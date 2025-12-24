/// Dynamic route example: api/users/[id].js -> /api/users/{id}
/// Usage:
/// - GET: curl http://localhost:3000/api/users/42
/// - POST: curl -X POST http://localhost:3000/api/users/42 -H "Content-Type: application/json" -d '{"action":"update"}'
/// - PUT: curl -X PUT http://localhost:3000/api/users/42 -H "Content-Type: application/json" -d '{"name":"New Name"}'
/// - DELETE: curl -X DELETE http://localhost:3000/api/users/42

function GET(reqString) {
    const req = JSON.parse(reqString);
    const userId = req.params.id;

    // Mock user data
    const users = {
        "1": { id: 1, name: "Ahmed", email: "ahmed@example.com" },
        "2": { id: 2, name: "Galal", email: "galal@example.com" },
        "42": { id: 42, name: "Test User", email: "test@example.com" }
    };

    const user = users[userId];

    if (user) {
        return JSON.stringify({
            status: 200,
            body: { user }
        });
    }

    return JSON.stringify({
        status: 404,
        body: { error: `User with id ${userId} not found` }
    });
}

function POST(reqString) {
    const req = JSON.parse(reqString);
    const userId = req.params.id;
    const data = req.body ? JSON.parse(req.body) : {};

    return JSON.stringify({
        status: 200,
        body: {
            message: `Action performed on user ${userId}`,
            action: data.action || "default",
            userId
        }
    });
}

function PUT(reqString) {
    const req = JSON.parse(reqString);
    const userId = req.params.id;
    const data = req.body ? JSON.parse(req.body) : {};

    return JSON.stringify({
        status: 200,
        body: {
            message: `User ${userId} updated`,
            updates: data,
            userId
        }
    });
}

function DELETE(reqString) {
    const req = JSON.parse(reqString);
    const userId = req.params.id;

    return JSON.stringify({
        status: 200,
        body: {
            message: `User ${userId} deleted`,
            userId
        }
    });
}

module.exports = { GET, POST, PUT, DELETE };
