const express = require('express');
const mongoose = require('mongoose');
const bodyParser = require('body-parser');
const cors = require('cors');
const WebSocket = require('ws');
const Todo = require('./models/todo');

const app = express();
const PORT = 3000;

// Middleware
app.use(cors());
app.use(bodyParser.json());

// Connect to MongoDB
mongoose
    .connect(process.env.MONGO_URI || 'mongodb://localhost:27017/todo-list', {
        useNewUrlParser: true,
        useUnifiedTopology: true,
    })
    .then(() => console.log('MongoDB connected'))
    .catch((err) => console.error(err));

// WebSocket Server
const wss = new WebSocket.Server({ noServer: true });
const clients = new Set();

wss.on('connection', (ws) => {
    clients.add(ws);
    ws.on('close', () => clients.delete(ws));
});

// Notify WebSocket clients
const notifyClients = (event, data) => {
    clients.forEach((client) => {
        if (client.readyState === WebSocket.OPEN) {
            client.send(JSON.stringify({ event, data }));
        }
    });
};

// RESTful API

app.get('/todos/:id', async (req, res) => {
    const todo = await Todo.findById(req.params.id);
    res.json(todo);
});


app.get('/todos', async (req, res) => {
    const todos = await Todo.find();
    res.json(todos);
});

app.post('/todos', async (req, res) => {
    const todo = new Todo(req.body);
    await todo.save();
    res.status(201).json(todo);
    notifyClients('todo_created', todo);
});

app.put('/todos/:id', async (req, res) => {
    const todo = await Todo.findByIdAndUpdate(req.params.id, req.body, { new: true });
    res.json(todo);
    notifyClients('todo_updated', todo);
});

app.delete('/todos/:id', async (req, res) => {
    const todo = await Todo.findByIdAndDelete(req.params.id);
    res.status(204).send();
    notifyClients('todo_deleted', { id: req.params.id });
});

// Start HTTP and WebSocket servers
const server = app.listen(PORT, () => console.log(`Server running on http://localhost:${PORT}`));
server.on('upgrade', (request, socket, head) => {
    wss.handleUpgrade(request, socket, head, (ws) => {
        wss.emit('connection', ws, request);
    });
});
