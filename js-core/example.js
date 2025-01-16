const {CoreSdk} = require('.');

const sdk = new CoreSdk();
sdk.todo.add("hello napi")
console.log(sdk.todo.list());
