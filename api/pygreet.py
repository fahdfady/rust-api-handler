# RUN: curl -X GET http://localhost:3000/api/pygreet
import json

def GET(req_string):
    return json.dumps({
        "status": 200,
        "body": {"message": "Hello from Python!"}
    })

# RUN: curl -X POST http://localhost:3000/api/pygreet -H "Content-Type: application/json" -d '{"name": "Fahd"}'
def POST(req_string):
    req = json.loads(req_string)
    data = json.loads(req["body"])
    name = data["name"]
    
    return json.dumps({
        "status": 200,
        "body": {"message": f"Hello, {name}! From Python"}
    })
