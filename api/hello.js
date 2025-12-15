/// RUN: curl -X GET http://localhost:3000/api/hello
function GET(reqString) {
  return JSON.stringify({
    status: 200,
    body: { message: "Hello from JavaScript!" }
  });
}

/// RUN: curl -X POST http://localhost:3000/api/hello -H "Content-Type: application/json" -d '{"name": "Fahd"}'
function POST(reqString) {
  const req = JSON.parse(reqString);
  const data = JSON.parse(req.body);
  const name = data.name;

  return JSON.stringify({
    status: 200,
    body: { message: `Hello, ${name}!` }
  });
}

/// RUN: curl -X PUT http://localhost:3000/api/hello -H "Content-Type: application/json" -d '{"name": "Fahd", "newName": "Ashour"}'
function PUT(reqString) {
  const req = JSON.parse(reqString);
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
function DELETE(reqString) {
  const req = JSON.parse(reqString);
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

module.exports = { GET, POST, PUT, DELETE };
