function GET() {
    return JSON.stringify({
        status: 200,
        body: { message: "Hello from JavaScript!" }
    });
}

function POST(req) {
    console.log(req);
    let body = request.body;
}