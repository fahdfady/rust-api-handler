# RUN: curl -X POST http://localhost:3000/api/portfolio/optimize -H "Content-Type: application/json" -d '{"assets": [{"symbol": "AAPL", "expected_return": 0.12, "risk": 0.18}, {"symbol": "GOOGL", "expected_return": 0.15, "risk": 0.22}, {"symbol": "MSFT", "expected_return": 0.10, "risk": 0.15}], "target_return": 0.12}'

import json

def optimize_portfolio(assets, target_return, max_risk=None):
    """Simple portfolio optimization using equal-weighted approach with constraints"""
    # In production, you'd use scipy.optimize or cvxpy for proper optimization
    # This is a simplified example showing the concept
    
    if not assets:
        return None
    
    # Sort by Sharpe-like ratio (return/risk)
    assets_sorted = sorted(assets, key=lambda x: x["expected_return"] / x["risk"] if x["risk"] > 0 else 0, reverse=True)
    
    # Simple equal-weight allocation
    n = len(assets_sorted)
    allocation = []
    
    for asset in assets_sorted:
        weight = 1.0 / n
        allocation.append({
            "symbol": asset["symbol"],
            "weight": round(weight * 100, 2),
            "expected_return": round(asset["expected_return"] * 100, 2),
            "risk": round(asset["risk"] * 100, 2)
        })
    
    # Calculate portfolio metrics
    portfolio_return = sum(a["expected_return"] * (a["weight"] / 100) for a in allocation)
    # Simplified portfolio risk (in reality, need covariance matrix)
    portfolio_risk = sum(assets_sorted[i]["risk"] * (1.0 / n) for i in range(n))
    
    return {
        "allocation": allocation,
        "portfolio_metrics": {
            "expected_return": round(portfolio_return * 100, 2),
            "expected_risk": round(portfolio_risk * 100, 2),
            "sharpe_ratio": round(portfolio_return / portfolio_risk if portfolio_risk > 0 else 0, 3)
        }
    }

def POST(req_string):
    """Optimize portfolio allocation"""
    try:
        req = json.loads(req_string)
        data = json.loads(req["body"])
        
        assets = data.get("assets", [])
        target_return = data.get("target_return", 0.10)
        max_risk = data.get("max_risk")
        
        if not assets:
            return json.dumps({
                "status": 400,
                "body": {"error": "assets array is required"}
            })
        
        result = optimize_portfolio(assets, target_return, max_risk)
        
        if not result:
            return json.dumps({
                "status": 400,
                "body": {"error": "Unable to optimize portfolio"}
            })
        
        return json.dumps({
            "status": 200,
            "body": {
                "message": "Portfolio optimization complete",
                "data": result,
                "note": "Using simplified equal-weight model. Production would use mean-variance optimization."
            }
        })
    except Exception as e:
        return json.dumps({
            "status": 500,
            "body": {"error": str(e)}
        })