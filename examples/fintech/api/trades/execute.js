// RUN: curl -X POST http://localhost:3000/api/trades/execute -H "Content-Type: application/json" -d '{"symbol": "AAPL", "type": "buy", "quantity": 10, "price": 175.50}'

function POST(req) {
    const data = JSON.parse(req.body);
    const { symbol, type, quantity, price, order_type = "market" } = data;

    // Validate required fields
    if (!symbol || !type || !quantity) {
        return JSON.stringify({
            status: 400,
            body: { error: "symbol, type, and quantity are required" }
        });
    }

    if (!["buy", "sell"].includes(type.toLowerCase())) {
        return JSON.stringify({
            status: 400,
            body: { error: "type must be 'buy' or 'sell'" }
        });
    }

    // Simulate trade execution
    const executionPrice = price || (Math.random() * 500 + 50);
    const commission = quantity * 0.005; // $0.005 per share
    const totalCost = type === "buy"
        ? (executionPrice * quantity) + commission
        : (executionPrice * quantity) - commission;

    const trade = {
        trade_id: `TRD-${Date.now()}-${Math.random().toString(36).substr(2, 9)}`,
        symbol,
        type: type.toLowerCase(),
        quantity,
        execution_price: parseFloat(executionPrice.toFixed(2)),
        total_cost: parseFloat(totalCost.toFixed(2)),
        commission: parseFloat(commission.toFixed(2)),
        order_type,
        status: "executed",
        executed_at: new Date().toISOString()
    };

    return JSON.stringify({
        status: 201,
        body: {
            message: "Trade executed successfully",
            data: trade
        }
    });
}

// RUN: curl -X GET http://localhost:3000/api/trades/execute
function GET(req) {
    // Get recent trades (simulated)
    const recentTrades = [
        {
            trade_id: "TRD-001",
            symbol: "AAPL",
            type: "buy",
            quantity: 10,
            execution_price: 175.50,
            executed_at: new Date(Date.now() - 3600000).toISOString()
        },
        {
            trade_id: "TRD-002",
            symbol: "GOOGL",
            type: "sell",
            quantity: 5,
            execution_price: 2950.00,
            executed_at: new Date(Date.now() - 7200000).toISOString()
        }
    ];

    return JSON.stringify({
        status: 200,
        body: {
            data: recentTrades,
            count: recentTrades.length
        }
    });
}

module.exports = { GET, POST };