function GET() {
    return JSON.stringify({
        status: 200,
        body: { message: "Hello from JavaScript!" }
    });
}


/// RUN `curl -X POST http://localhost:3000/api/hello -H "Content-Type: application/json" -d '{"name": "fahd"}'`
function POST(req) {
    const data = JSON.parse(req.body);
    const name = data.name;
    return JSON.stringify({
        status: 200,
        body: { message: `Hello, ${name}!` }
    });
}