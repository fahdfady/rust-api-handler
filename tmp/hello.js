const request_1763637646175290971 = { "url": "/api/hello", "headers": { "host": "localhost:3000", "user-agent": "curl/8.16.0", "accept": "*/*" }, "method": "GET", "body": null, "params": {}, "query": {} };
const method_1763637646175290971 = request_1763637646175290971.method;

/// RUN: curl -X GET http://localhost:3000/api/hello
function GET() {
  return JSON.stringify({
    status: 200,
    body: { message: "Hello from JavaScript!" }
  });
}

/// RUN: curl -X POST http://localhost:3000/api/hello -H "Content-Type: application/json" -d '{"name": "Fahd"}'
function POST(req) {
  const data = JSON.parse(req.body);
  const name = data.name;

  return JSON.stringify({
    status: 200,
    body: { message: `Hello, ${name}!` }
  });
}

/// RUN: curl -X PUT http://localhost:3000/api/hello -H "Content-Type: application/json" -d '{"name": "Fahd", "newName": "Ashour"}'
function PUT(req) {
  const data = JSON.parse(req.body);
  const { name, newName } = data;

  if (!name || !newName) {
    return JSON.stringify({
      status: 400,
      body: { error: "Both 'name' and 'newName' fields are required." }
    });
  }

  return JSON.stringify({
    status: 200,
    body: { message: `Updated name from ${name} to ${newName}` }
  });
}

/// RUN: curl -X DELETE http://localhost:3000/api/hello -H "Content-Type: application/json" -d '{"name": "Fahd"}'
function DELETE(req) {
  const data = JSON.parse(req.body);
  const { name } = data;

  if (!name) {
    return JSON.stringify({
      status: 400,
      body: { error: "'name' field is required." }
    });
  }

  return JSON.stringify({
    status: 200,
    body: { message: `Deleted user ${name}` }
  });
}

function handler_1763637646175290971() {
  const handlerFn = globalThis[method_1763637646175290971];
  if (typeof handlerFn !== 'function') {
    return JSON.stringify({
      status: 405,
      body: { error: 'Method ' + method_1763637646175290971 + ' not allowed' }
    });
  }
  const result = handlerFn(request_1763637646175290971);
  return JSON.stringify(result);
}

module.exports = { handler_1763637646175290971: handler_1763637646175290971 }; 
