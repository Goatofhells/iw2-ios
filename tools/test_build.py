import pathlib
import plistlib
import struct
import subprocess
import sys
import tempfile
import unittest
import zipfile


class build(unittest.TestCase):
    def test_config(self):
        root = pathlib.Path(__file__).resolve().parents[1]
        text = (root / "ios/project.yml").read_text()
        for item in ['iOS: "15.0"', 'SWIFT_VERSION: "5.0"', "-llauncher", "SWIFT_OBJC_BRIDGING_HEADER", 'CODE_SIGNING_ALLOWED: "NO"']:
            self.assertIn(item, text)
        text = (root / ".github/workflows/ios-release.yml").read_text()
        for item in ["workflow_dispatch:", "macos-15", "aarch64-apple-ios", "cargo test", "xcodebuild", "verify_ipa.py", "upload-artifact", "prerelease"]:
            self.assertIn(item, text)

    def test_ipa(self):
        script = pathlib.Path(__file__).with_name("verify_ipa.py")
        with tempfile.TemporaryDirectory() as temp:
            for mode in ["valid", "missing", "x86", "truncated", "target"]:
                path = pathlib.Path(temp) / f"{mode}.ipa"
                info = {"CFBundleExecutable": "app", "CFBundleIdentifier": "com.example.test", "CFBundlePackageType": "APPL", "MinimumOSVersion": "14.0" if mode == "target" else "15.0", "UILaunchScreen": {}}
                data = struct.pack("<IIIIIIII", 0xFEEDFACF, 7 if mode == "x86" else 0x0100000C, 0, 2, 0, 0, 0, 0)
                with zipfile.ZipFile(path, "w") as ipa:
                    ipa.writestr("Payload/Test.app/Info.plist", plistlib.dumps(info))
                    if mode != "missing":
                        ipa.writestr("Payload/Test.app/app", b"bad" if mode == "truncated" else data)
                res = subprocess.run([sys.executable, str(script), str(path)], capture_output=True)
                self.assertEqual(res.returncode == 0, mode == "valid", res.stderr.decode())


if __name__ == "__main__":
    unittest.main()
