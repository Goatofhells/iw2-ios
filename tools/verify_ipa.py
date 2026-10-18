import pathlib
import plistlib
import struct
import sys
import zipfile

with zipfile.ZipFile(sys.argv[1]) as ipa:
    names = ipa.namelist()
    infos = [name for name in names if name.startswith("Payload/") and name.count("/") == 2 and name.endswith(".app/Info.plist")]
    if len(infos) != 1:
        raise ValueError("Expected exactly one application Info.plist")
    info = plistlib.loads(ipa.read(infos[0]))
    for key in ["CFBundleExecutable", "CFBundleIdentifier", "CFBundlePackageType", "MinimumOSVersion"]:
        if not info.get(key):
            raise ValueError(f"Missing {key}")
    if info["CFBundlePackageType"] != "APPL" or info["MinimumOSVersion"] != "15.0":
        raise ValueError("Invalid app type or deployment target")
    exe = info["CFBundleExecutable"]
    if pathlib.PurePosixPath(exe).name != exe:
        raise ValueError("Invalid executable path")
    root = infos[0].rsplit("/", 1)[0]
    data = ipa.read(f"{root}/{exe}")
    if len(data) < 32:
        raise ValueError("Executable is truncated")
    magic, cpu, sub, kind = struct.unpack("<IIII", data[:16])
    if magic != 0xFEEDFACF or cpu != 0x0100000C or kind != 2:
        raise ValueError("Expected ARM64 Mach-O executable")
    if not any(name.startswith(f"{root}/") and name.endswith(".storyboardc/") for name in names) and "UILaunchScreen" not in info:
        raise ValueError("Launch screen missing")
    for name in names:
        path = pathlib.PurePosixPath(name)
        if path.is_absolute() or ".." in path.parts or "\\" in name:
            raise ValueError("Unsafe IPA entry")
    print(f"Verified unsigned IPA structure: {info['CFBundleIdentifier']} ARM64 iOS {info['MinimumOSVersion']}")
