# RUN: curl -X POST http://localhost:3000/api/portfolio/risk -H "Content-Type: application/json" -d '{"returns": [0.02, -0.01, 0.03, -0.02, 0.04, 0.01, -0.03], "portfolio_value": 100000}'

import json
import math

def calculate_risk_metrics(returns, portfolio_value):
    """Calculate portfolio risk metrics including VaR, Sharpe ratio, etc."""
    if not returns or len(returns) < 2:
        return None
    
    n = len(returns)
    mean_return = sum(returns) / n
    
    # Calculate standard deviation (volatility)
    variance = sum((r - mean_return) ** 2 for r in returns) / (n - 1)
    std_dev = math.sqrt(variance)
    
    # Annualized metrics (assuming daily returns)
    annual_return = mean_return * 252
    annual_volatility = std_dev * math.sqrt(252)
    
    # Sharpe Ratio (assuming 2% risk-free rate)
    risk_free_rate = 0.02
    sharpe_ratio = (annual_return - risk_free_rate) / annual_volatility if annual_volatility > 0 else 0
    
    # Value at Risk (95% confidence)
    sorted_returns = sorted(returns)
    var_index = int(len(sorted_returns) * 0.05)
    var_95 = abs(sorted_returns[var_index]) * portfolio_value
    
    # Maximum Drawdown
    cumulative = [1]
    for r in returns:
        cumulative.append(cumulative[-1] * (1 + r))
    
    max_drawdown = 0
    peak = cumulative[0]
    for value in cumulative:
        if value > peak:
            peak = value
        drawdown = (peak - value) / peak
        max_drawdown = max(max_drawdown, drawdown)
    
    return {
        "mean_daily_return": round(mean_return * 100, 4),
        "daily_volatility": round(std_dev * 100, 4),
        "annual_return": round(annual_return * 100, 2),
        "annual_volatility": round(annual_volatility * 100, 2),
        "sharpe_ratio": round(sharpe_ratio, 3),
        "value_at_risk_95": round(var_95, 2),
        "max_drawdown": round(max_drawdown * 100, 2)
    }

def POST(req_string):
    """Calculate risk metrics for portfolio"""
    try:
        req = json.loads(req_string)
        data = json.loads(req["body"])
        
        returns = data.get("returns", [])
        portfolio_value = data.get("portfolio_value", 0)
        
        if not returns:
            return json.dumps({
                "status": 400,
                "body": {"error": "returns array is required"}
            })
        
        metrics = calculate_risk_metrics(returns, portfolio_value)
        
        if not metrics:
            return json.dumps({
                "status": 400,
                "body": {"error": "Insufficient data for risk calculation"}
            })
        
        return json.dumps({
            "status": 200,
            "body": {
                "message": "Risk analysis complete",
                "data": metrics
            }
        })
    except Exception as e:
        return json.dumps({
            "status": 500,
            "body": {"error": str(e)}
        })
