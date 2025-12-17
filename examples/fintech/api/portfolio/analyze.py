# RUN: curl -X POST http://localhost:3000/api/portfolio/analyze -H "Content-Type: application/json" -d '{"holdings": [{"symbol": "AAPL", "shares": 10, "avg_price": 150}, {"symbol": "GOOGL", "shares": 5, "avg_price": 2800}], "current_prices": {"AAPL": 175, "GOOGL": 2950}}'

import json
import math

def calculate_portfolio_metrics(holdings, current_prices):
    """Calculate comprehensive portfolio metrics"""
    total_value = 0
    total_cost = 0
    positions = []
    
    for holding in holdings:
        symbol = holding["symbol"]
        shares = holding["shares"]
        avg_price = holding["avg_price"]
        current_price = current_prices.get(symbol, avg_price)
        
        cost_basis = shares * avg_price
        current_value = shares * current_price
        gain_loss = current_value - cost_basis
        gain_loss_pct = (gain_loss / cost_basis * 100) if cost_basis > 0 else 0
        
        total_value += current_value
        total_cost += cost_basis
        
        positions.append({
            "symbol": symbol,
            "shares": shares,
            "avg_price": round(avg_price, 2),
            "current_price": round(current_price, 2),
            "cost_basis": round(cost_basis, 2),
            "current_value": round(current_value, 2),
            "gain_loss": round(gain_loss, 2),
            "gain_loss_pct": round(gain_loss_pct, 2),
            "weight": 0  # Will calculate after total
        })
    
    # Calculate weights
    for position in positions:
        position["weight"] = round((position["current_value"] / total_value * 100) if total_value > 0 else 0, 2)
    
    total_gain_loss = total_value - total_cost
    total_return_pct = (total_gain_loss / total_cost * 100) if total_cost > 0 else 0
    
    return {
        "positions": positions,
        "summary": {
            "total_value": round(total_value, 2),
            "total_cost": round(total_cost, 2),
            "total_gain_loss": round(total_gain_loss, 2),
            "total_return_pct": round(total_return_pct, 2)
        }
    }

def POST(req_string):
    """Analyze portfolio performance and calculate metrics"""
    try:
        req = json.loads(req_string)
        data = json.loads(req["body"])
        
        holdings = data.get("holdings", [])
        current_prices = data.get("current_prices", {})
        
        if not holdings:
            return json.dumps({
                "status": 400,
                "body": {"error": "holdings array is required"}
            })
        
        metrics = calculate_portfolio_metrics(holdings, current_prices)
        
        return json.dumps({
            "status": 200,
            "body": {
                "message": "Portfolio analysis complete",
                "data": metrics
            }
        })
    except Exception as e:
        return json.dumps({
            "status": 500,
            "body": {"error": str(e)}
        })

