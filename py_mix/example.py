from todo_app import TodoSdk, Util

print(Util.show_computer_info())

sdk = TodoSdk()
todo = sdk.todo

print(todo.add(title="create rust hello world project", completed=False))
print(todo.list())
