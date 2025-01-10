from todo_app import CoreSdk, Util

print(Util.show_computer_info())

sdk = CoreSdk()

todo = sdk.todo

print(todo.add(title="create rust hello world project", completed=False))
print(todo.list())
