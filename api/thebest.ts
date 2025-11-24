/// RUN: curl -X GET http://localhost:3000/api/the-best
// @ts-expect-error dwqeqweqw
function GET(): string {

  return JSON.stringify({
    status: 200,
    body: {
      message: "Fahd is the best"
    }
  });
}
