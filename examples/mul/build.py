import ctypes
import subprocess

def main():
    process = subprocess.Popen(
        ["cargo", "build", "--release"],
        stdout=subprocess.PIPE,
        stderr=subprocess.STDOUT,
        text=True
    )

    for line in process.stdout:
        print(line, end="")

    process.wait()

    if process.returncode != 0:
        return

    lib_path = "./target/release/libmul.so"
    lib = ctypes.CDLL(lib_path)

    lib.fmi2ModelDescription.restype = ctypes.POINTER(ctypes.c_char)
    lib.fmi2ModelDescription.argtypes = []

    lib.fmi2FreeModelDescription.restype = None
    lib.fmi2FreeModelDescription.argtypes = [ctypes.POINTER(ctypes.c_char)]

    ptr = lib.fmi2ModelDescription()

    try:
        xml = ctypes.cast(ptr, ctypes.c_char_p).value
        print(xml.decode("utf-8"))
    finally:
        lib.fmi2FreeModelDescription(ptr)
    
if __name__ == "__main__":
    main()
