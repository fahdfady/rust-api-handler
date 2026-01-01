# RUN: curl -X POST http://localhost:3000/api/embeddings/generate -H "Content-Type: application/json" -d '{"texts": ["Hello world", "AI is amazing"], "model": "text-embedding-ada-002"}'
"""
Text Embeddings Generation API Handler

This demonstrates ML inference directly in your API routes.
In production, you would use:
- sentence-transformers
- OpenAI embeddings
- Hugging Face models
- FAISS/Pinecone for vector storage
"""
import json
from datetime import datetime
import hashlib

# In production:
# from sentence_transformers import SentenceTransformer
# import openai
# from transformers import AutoTokenizer, AutoModel

# Supported embedding models
EMBEDDING_MODELS = {
    "text-embedding-ada-002": {
        "provider": "openai",
        "dimensions": 1536,
        "max_tokens": 8191
    },
    "text-embedding-3-small": {
        "provider": "openai", 
        "dimensions": 1536,
        "max_tokens": 8191
    },
    "all-MiniLM-L6-v2": {
        "provider": "sentence-transformers",
        "dimensions": 384,
        "max_tokens": 512
    },
    "bge-large-en-v1.5": {
        "provider": "huggingface",
        "dimensions": 1024,
        "max_tokens": 512
    }
}


def GET(req_string):
    """List available embedding models"""
    return json.dumps({
        "status": 200,
        "body": {
            "models": EMBEDDING_MODELS,
            "default_model": "text-embedding-ada-002",
            "service": "embeddings-generator"
        }
    })


def POST(req_string):
    """
    Generate embeddings for input texts
    
    In production with sentence-transformers:
    
    from sentence_transformers import SentenceTransformer
    model = SentenceTransformer('all-MiniLM-L6-v2')
    embeddings = model.encode(texts)
    
    Or with OpenAI:
    
    import openai
    response = openai.Embedding.create(
        input=texts,
        model="text-embedding-ada-002"
    )
    """
    req = json.loads(req_string)
    body = json.loads(req.get("body", "{}"))
    
    texts = body.get("texts", [])
    model = body.get("model", "text-embedding-ada-002")
    
    # Handle single text input
    if isinstance(texts, str):
        texts = [texts]
    
    if not texts:
        return json.dumps({
            "status": 400,
            "body": {"error": "Missing 'texts' in request body. Provide a string or array of strings."}
        })
    
    if model not in EMBEDDING_MODELS:
        return json.dumps({
            "status": 400,
            "body": {
                "error": f"Unknown model: {model}",
                "available_models": list(EMBEDDING_MODELS.keys())
            }
        })
    
    model_info = EMBEDDING_MODELS[model]
    dimensions = model_info["dimensions"]
    
    # ============================================
    # In production, replace this with actual embedding generation:
    #
    # from sentence_transformers import SentenceTransformer
    # model = SentenceTransformer('all-MiniLM-L6-v2')
    # embeddings = model.encode(texts).tolist()
    #
    # Key benefit: Your Python ML models run in the SAME
    # process as JavaScript business logic - no HTTP overhead!
    # ============================================
    
    # Generate simulated embeddings (deterministic based on text hash)
    embeddings = []
    for text in texts:
        # Create a deterministic "embedding" based on text hash
        text_hash = hashlib.sha256(text.encode()).hexdigest()
        # Generate pseudo-random but consistent values
        embedding = []
        for i in range(dimensions):
            # Use hash to generate consistent float values between -1 and 1
            hash_segment = text_hash[(i * 2) % len(text_hash):((i * 2) % len(text_hash)) + 2]
            value = (int(hash_segment, 16) / 255.0) * 2 - 1
            embedding.append(round(value, 6))
        embeddings.append(embedding)
    
    return json.dumps({
        "status": 200,
        "body": {
            "object": "list",
            "data": [
                {
                    "object": "embedding",
                    "index": i,
                    "text": texts[i][:50] + "..." if len(texts[i]) > 50 else texts[i],
                    "embedding": embeddings[i][:10],  # Return first 10 dims for readability
                    "full_dimensions": dimensions
                }
                for i in range(len(texts))
            ],
            "model": model,
            "usage": {
                "prompt_tokens": sum(len(t.split()) for t in texts),
                "total_tokens": sum(len(t.split()) for t in texts)
            }
        }
    })
