from todo_app import CoreSdk, Util
import time

sdk = CoreSdk()

print(dir(sdk))

print(Util.show_computer_info())

def on_changed(content):
    print("==changed==")
    print(content)
    print("========")

sdk.set_on_changed(on_changed)

def on_error(content):
    print("==error==")
    print(content)
    print("========")

sdk.set_on_error(on_error)

def on_disconnected(content):
    print("==disconnected==")
    print(content)
    print("========")

sdk.set_on_disconnected(on_disconnected)

sdk.connect_websocket()

todo = sdk.todo

print(todo.add(title="create rust hello world project", completed=False))
# print(todo.list())

time.sleep(20)