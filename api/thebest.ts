/// RUN: curl -X GET http://localhost:3000/api/the-best
export function GET(): string {
  return JSON.stringify({
    status: 200,
    body: {
      message: "Fahd is the best"
    }
  });
}

