
// RUN: curl -X GET http://localhost:3000/api/watchlist/manage
// Manage user watchlists with fast CRUD operations

const watchlists = new Map(); // In-memory storage (use DB in production)

function GET(req) {
    const allWatchlists = Array.from(watchlists.values());

    return JSON.stringify({
        status: 200,
        body: {
            data: allWatchlists,
            count: allWatchlists.length
        }
    });
}

// RUN: curl -X POST http://localhost:3000/api/watchlist/manage -H "Content-Type: application/json" -d '{"name": "Tech Stocks", "symbols": ["AAPL", "GOOGL", "MSFT"]}'
function POST(req) {
    const data = JSON.parse(req.body);
    const { name, symbols = [] } = data;

    if (!name) {
        return JSON.stringify({
            status: 400,
            body: { error: "name is required" }
        });
    }

    const watchlistId = `WL-${Date.now()}`;
    const watchlist = {
        id: watchlistId,
        name,
        symbols,
        created_at: new Date().toISOString(),
        updated_at: new Date().toISOString()
    };

    watchlists.set(watchlistId, watchlist);

    return JSON.stringify({
        status: 201,
        body: {
            message: "Watchlist created successfully",
            data: watchlist
        }
    });
}

// RUN: curl -X PUT http://localhost:3000/api/watchlist/manage -H "Content-Type: application/json" -d '{"id": "WL-123", "symbols": ["AAPL", "TSLA", "NVDA"]}'
function PUT(req) {
    const data = JSON.parse(req.body);
    const { id, name, symbols } = data;

    if (!id) {
        return JSON.stringify({
            status: 400,
            body: { error: "id is required" }
        });
    }

    const watchlist = watchlists.get(id);
    if (!watchlist) {
        return JSON.stringify({
            status: 404,
            body: { error: "Watchlist not found" }
        });
    }

    if (name) watchlist.name = name;
    if (symbols) watchlist.symbols = symbols;
    watchlist.updated_at = new Date().toISOString();

    watchlists.set(id, watchlist);

    return JSON.stringify({
        status: 200,
        body: {
            message: "Watchlist updated successfully",
            data: watchlist
        }
    });
}

// RUN: curl -X DELETE http://localhost:3000/api/watchlist/manage -H "Content-Type: application/json" -d '{"id": "WL-123"}'
function DELETE(req) {
    const data = JSON.parse(req.body);
    const { id } = data;

    if (!id) {
        return JSON.stringify({
            status: 400,
            body: { error: "id is required" }
        });
    }

    const deleted = watchlists.delete(id);

    if (!deleted) {
        return JSON.stringify({
            status: 404,
            body: { error: "Watchlist not found" }
        });
    }

    return JSON.stringify({
        status: 200,
        body: { message: `Watchlist ${id} deleted successfully` }
    });
}

module.exports = { GET, POST, PUT, DELETE };


// api/alerts/notifications.js
// RUN: curl -X POST http://localhost:3000/api/alerts/notifications -H "Content-Type: application/json" -d '{"symbol": "AAPL", "condition": "above", "target_price": 180}'
// Real-time price alert management

const alerts = [];

function GET(req) {
    return JSON.stringify({
        status: 200,
        body: {
            data: alerts,
            count: alerts.length,
            message: "Active price alerts"
        }
    });
}

function POST(req) {
    const data = JSON.parse(req.body);
    const { symbol, condition, target_price, user_id = "user_default" } = data;

    if (!symbol || !condition || !target_price) {
        return JSON.stringify({
            status: 400,
            body: { error: "symbol, condition, and target_price are required" }
        });
    }

    if (!["above", "below"].includes(condition)) {
        return JSON.stringify({
            status: 400,
            body: { error: "condition must be 'above' or 'below'" }
        });
    }

    const alert = {
        alert_id: `ALT-${Date.now()}`,
        user_id,
        symbol,
        condition,
        target_price,
        status: "active",
        created_at: new Date().toISOString()
    };

    alerts.push(alert);

    return JSON.stringify({
        status: 201,
        body: {
            message: "Price alert created successfully",
            data: alert
        }
    });
}

function DELETE(req) {
    const data = JSON.parse(req.body);
    const { alert_id } = data;

    if (!alert_id) {
        return JSON.stringify({
            status: 400,
            body: { error: "alert_id is required" }
        });
    }

    const index = alerts.findIndex(a => a.alert_id === alert_id);

    if (index === -1) {
        return JSON.stringify({
            status: 404,
            body: { error: "Alert not found" }
        });
    }

    alerts.splice(index, 1);

    return JSON.stringify({
        status: 200,
        body: { message: `Alert ${alert_id} deleted successfully` }
    });
}

module.exports = { GET, POST, DELETE };