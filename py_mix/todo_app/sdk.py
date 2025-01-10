from ._todo_sdk import CoreSdk
import platform
import psutil

class TodoSdk(CoreSdk):
    """
    fubon sdk for api trading
    """

    def show_computer_info(self):
        # 系統相關資訊
        system = platform.system()
        node_name = platform.node()
        release = platform.release()
        version = platform.version()
        architecture = platform.architecture()[0]
        processor = platform.processor()
        cpu_count = psutil.cpu_count(logical=True)
        memory = psutil.virtual_memory()
        total_memory = memory.total / (1024 ** 3)  # 換算為GB

        # 磁碟相關資訊
        disk = psutil.disk_usage('/')
        total_disk = disk.total / (1024 ** 3)  # 換算為GB

        print(f"=== Computer Info ===")
        print(f"System: {system}")
        print(f"Node Name: {node_name}")
        print(f"Release: {release}")
        print(f"Version: {version}")
        print(f"Architecture: {architecture}")
        print(f"Processor: {processor}")
        print(f"CPU Cores: {cpu_count}")
        print(f"Total Memory: {total_memory:.2f} GB")
        print(f"Total Disk Space: {total_disk:.2f} GB")


