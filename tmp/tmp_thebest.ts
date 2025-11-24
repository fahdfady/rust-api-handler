
/// RUN: curl -X GET http://localhost:3000/api/the-best
function GET(): string {

  return JSON.stringify({
    status: 200,
    body: {
      message: "Fahd is the best"
    }
  });
}


const handlers = { GET, POST, PUT, DELETE };

const request = { "url": "/api/thebest", "headers": { "host": "localhost:3000", "user-agent": "curl/8.16.0", "accept": "*/*" }, "method": "GET", "body": null, "params": {}, "query": {} };
const method = request.method;

function handler() {
  console.log(method);
  const handlerFn = handlers[method];
  console.log(handlerFn);
  if (typeof handlerFn !== 'function') {
    return JSON.stringify({
      status: 405,
      body: { error: 'Method ' + method + ' not allowed' }
    });
  }
  const result = handlerFn(request);
  return result;
}

module.exports = { handler }; 
