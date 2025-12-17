// RUN: curl -X GET http://localhost:3000/api/market/prices
// Real-time price streaming endpoint (simulated)

function GET(req) {
    // Simulate real-time market prices
    const symbols = ["AAPL", "GOOGL", "MSFT", "AMZN", "TSLA", "META", "NVDA", "JPM"];

    const prices = symbols.map(symbol => {
        const basePrice = {
            "AAPL": 175.50,
            "GOOGL": 2950.00,
            "MSFT": 380.25,
            "AMZN": 3100.00,
            "TSLA": 245.80,
            "META": 485.20,
            "NVDA": 875.30,
            "JPM": 195.45
        }[symbol] || 100;

        // Simulate small price fluctuation
        const change = (Math.random() - 0.5) * basePrice * 0.02;
        const currentPrice = basePrice + change;
        const changePercent = (change / basePrice) * 100;

        return {
            symbol,
            price: parseFloat(currentPrice.toFixed(2)),
            change: parseFloat(change.toFixed(2)),
            change_percent: parseFloat(changePercent.toFixed(2)),
            volume: Math.floor(Math.random() * 10000000) + 1000000,
            timestamp: new Date().toISOString(),
            bid: parseFloat((currentPrice - 0.05).toFixed(2)),
            ask: parseFloat((currentPrice + 0.05).toFixed(2))
        };
    });

    return JSON.stringify({
        status: 200,
        body: {
            data: prices,
            market_status: "open",
            last_updated: new Date().toISOString()
        }
    });
}

// RUN: curl -X POST http://localhost:3000/api/market/prices -H "Content-Type: application/json" -d '{"symbols": ["AAPL", "TSLA"]}'
function POST(req) {
    const data = JSON.parse(req.body);
    const symbols = data.symbols || [];

    if (!Array.isArray(symbols) || symbols.length === 0) {
        return JSON.stringify({
            status: 400,
            body: { error: "symbols array is required" }
        });
    }

    const prices = symbols.map(symbol => {
        const basePrice = Math.random() * 1000 + 50;
        const change = (Math.random() - 0.5) * basePrice * 0.03;

        return {
            symbol,
            price: parseFloat((basePrice + change).toFixed(2)),
            change: parseFloat(change.toFixed(2)),
            change_percent: parseFloat(((change / basePrice) * 100).toFixed(2)),
            timestamp: new Date().toISOString()
        };
    });

    return JSON.stringify({
        status: 200,
        body: { data: prices }
    });
}

module.exports = { GET, POST };

