import os
import platform

import sys
from pathlib import Path

def get_binary_name():
    system = platform.system().lower()
    machine = platform.machine().lower()

    # Map CPU architecture
    arch = "arm64" if machine in ("arm64", "aarch64") else "x86_64"

    # Map OS name
    if system == "windows":
        os_name = "windows"
        extension = ""  # Note: removed .exe since it's part of the binary name
    elif system == "darwin":
        os_name = "darwin"
        extension = ""
    else:  # Assume Linux for any other OS
        os_name = "linux"
        extension = ""

    return f"athira-{os_name}-{arch}{extension}"



def main():
    # Get the directory containing the binary
    bin_dir = Path(__file__).parent / "bin"
    binary_name = get_binary_name()
    binary_path = str(bin_dir / binary_name)

    if not os.path.exists(binary_path):
        print(f"Error: Binary not found for your platform: {binary_path}", file=sys.stderr)
        sys.exit(1)

    os.execv(binary_path, [binary_path] + sys.argv[1:])

if __name__ == "__main__":
    main()
