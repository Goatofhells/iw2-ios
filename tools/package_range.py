import os
import pathlib
import plistlib
import shutil
import subprocess
import sys

target = sys.argv[1]
sdk = "iphonesimulator" if target.endswith("-sim") or target.startswith("x86_64") else "iphoneos"
dest = pathlib.Path(sys.argv[2]) / "PocketRange.app"
dest.mkdir(parents=True, exist_ok=True)
shutil.copy2(f"target/{target}/release/range", dest / "range")
os.chmod(dest / "range", 0o755)
info = {
    "CFBundleExecutable": "range",
    "CFBundleIdentifier": "com.iw4pocket.range",
    "CFBundleName": "PocketRange",
    "CFBundleDisplayName": "Pocket Range",
    "CFBundlePackageType": "APPL",
    "CFBundleVersion": "2",
    "CFBundleShortVersionString": "0.2.0",
    "MinimumOSVersion": "15.0",
    "CFBundleSupportedPlatforms": ["iPhoneSimulator" if sdk == "iphonesimulator" else "iPhoneOS"],
    "UIDeviceFamily": [1, 2],
    "UIRequiresFullScreen": True,
    "UIStatusBarHidden": True,
    "UILaunchScreen": {},
    "UISupportedInterfaceOrientations": ["UIInterfaceOrientationLandscapeLeft", "UIInterfaceOrientationLandscapeRight"],
    "CFBundleURLTypes": [{"CFBundleURLName": "Pocket Range", "CFBundleURLSchemes": ["pocketrange"]}],
    "ITSAppUsesNonExemptEncryption": False,
    "DTPlatformName": sdk,
    "DTSDKName": sdk + subprocess.check_output(["xcrun", "--sdk", sdk, "--show-sdk-version"], text=True).strip(),
}
(dest / "Info.plist").write_bytes(plistlib.dumps(info))
shutil.copy2("build/range-licenses.txt", dest / "ThirdPartyNotices.txt")
if sdk == "iphonesimulator":
    subprocess.run(["codesign", "--force", "--sign", "-", str(dest)], check=True)
print(dest)
