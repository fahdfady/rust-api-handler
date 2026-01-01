/// RUN: curl -X GET http://localhost:3000/api/models/list
/// RUN: curl -X POST http://localhost:3000/api/models/list -H "Content-Type: application/json" -d '{"name": "custom-model", "type": "chat"}'
/**
 * Model Registry API Handler (JavaScript)
 * 
 * This demonstrates using JavaScript for business logic alongside
 * Python ML code. JavaScript excels at:
 * - API orchestration
 * - Data validation
 * - Business rules
 * - Database operations
 * - Caching logic
 */

// Simulated model registry (in production, this would be in a database)
const MODEL_REGISTRY = {
  "gpt-4": {
    id: "gpt-4",
    name: "GPT-4",
    provider: "openai",
    type: "chat",
    context_length: 128000,
    pricing: { input: 0.03, output: 0.06 },
    capabilities: ["chat", "reasoning", "code", "analysis"],
    status: "active"
  },
  "gpt-3.5-turbo": {
    id: "gpt-3.5-turbo", 
    name: "GPT-3.5 Turbo",
    provider: "openai",
    type: "chat",
    context_length: 16385,
    pricing: { input: 0.0005, output: 0.0015 },
    capabilities: ["chat", "code"],
    status: "active"
  },
  "claude-3-opus": {
    id: "claude-3-opus",
    name: "Claude 3 Opus",
    provider: "anthropic",
    type: "chat",
    context_length: 200000,
    pricing: { input: 0.015, output: 0.075 },
    capabilities: ["chat", "reasoning", "code", "analysis", "vision"],
    status: "active"
  },
  "llama-2-70b": {
    id: "llama-2-70b",
    name: "Llama 2 70B",
    provider: "meta",
    type: "chat",
    context_length: 4096,
    pricing: { input: 0.0, output: 0.0 },
    capabilities: ["chat", "code"],
    status: "active"
  },
  "text-embedding-ada-002": {
    id: "text-embedding-ada-002",
    name: "Ada Embedding",
    provider: "openai",
    type: "embedding",
    dimensions: 1536,
    pricing: { input: 0.0001 },
    capabilities: ["embedding"],
    status: "active"
  }
};

/**
 * GET /api/models/list
 * List all available models with optional filtering
 */
function GET(reqString) {
  const req = JSON.parse(reqString);
  const query = req.query || {};
  
  let models = Object.values(MODEL_REGISTRY);
  
  // Filter by provider
  if (query.provider) {
    models = models.filter(m => m.provider === query.provider);
  }
  
  // Filter by type
  if (query.type) {
    models = models.filter(m => m.type === query.type);
  }
  
  // Filter by capability
  if (query.capability) {
    models = models.filter(m => m.capabilities.includes(query.capability));
  }
  
  // Filter by status
  if (query.status) {
    models = models.filter(m => m.status === query.status);
  }
  
  return JSON.stringify({
    status: 200,
    body: {
      object: "list",
      data: models,
      total: models.length,
      filters_applied: Object.keys(query).length > 0 ? query : null,
      providers: [...new Set(Object.values(MODEL_REGISTRY).map(m => m.provider))],
      types: [...new Set(Object.values(MODEL_REGISTRY).map(m => m.type))]
    }
  });
}

/**
 * POST /api/models/list
 * Register a new custom model
 */
function POST(reqString) {
  const req = JSON.parse(reqString);
  const body = JSON.parse(req.body || "{}");
  
  const { name, type, provider, context_length, capabilities } = body;
  
  if (!name) {
    return JSON.stringify({
      status: 400,
      body: { error: "Missing required field: 'name'" }
    });
  }
  
  if (!type) {
    return JSON.stringify({
      status: 400, 
      body: { error: "Missing required field: 'type'. Use 'chat' or 'embedding'" }
    });
  }
  
  // Generate model ID
  const modelId = `custom-${name.toLowerCase().replace(/\s+/g, '-')}-${Date.now()}`;
  
  const newModel = {
    id: modelId,
    name: name,
    provider: provider || "custom",
    type: type,
    context_length: context_length || 4096,
    pricing: { input: 0.0, output: 0.0 },
    capabilities: capabilities || [type],
    status: "pending",
    created_at: new Date().toISOString(),
    custom: true
  };
  
  // In production, save to database
  // await db.models.insert(newModel);
  
  return JSON.stringify({
    status: 201,
    body: {
      message: "Model registered successfully",
      model: newModel,
      next_steps: [
        "Upload model weights to /api/models/upload",
        "Configure inference endpoint",
        "Run validation tests"
      ]
    }
  });
}

/**
 * PUT /api/models/list  
 * Update model configuration
 */
function PUT(reqString) {
  const req = JSON.parse(reqString);
  const body = JSON.parse(req.body || "{}");
  
  const { id, updates } = body;
  
  if (!id) {
    return JSON.stringify({
      status: 400,
      body: { error: "Missing required field: 'id'" }
    });
  }
  
  if (!MODEL_REGISTRY[id]) {
    return JSON.stringify({
      status: 404,
      body: { error: `Model not found: ${id}` }
    });
  }
  
  // Simulate update
  const updatedModel = {
    ...MODEL_REGISTRY[id],
    ...updates,
    updated_at: new Date().toISOString()
  };
  
  return JSON.stringify({
    status: 200,
    body: {
      message: "Model updated successfully",
      model: updatedModel
    }
  });
}

/**
 * DELETE /api/models/list
 * Remove a custom model from registry
 */
function DELETE(reqString) {
  const req = JSON.parse(reqString);
  const body = JSON.parse(req.body || "{}");
  
  const { id } = body;
  
  if (!id) {
    return JSON.stringify({
      status: 400,
      body: { error: "Missing required field: 'id'" }
    });
  }
  
  const model = MODEL_REGISTRY[id];
  
  if (!model) {
    return JSON.stringify({
      status: 404,
      body: { error: `Model not found: ${id}` }
    });
  }
  
  if (!model.custom) {
    return JSON.stringify({
      status: 403,
      body: { error: "Cannot delete built-in models. Only custom models can be removed." }
    });
  }
  
  return JSON.stringify({
    status: 200,
    body: {
      message: "Model deleted successfully",
      deleted_model_id: id
    }
  });
}

module.exports = { GET, POST, PUT, DELETE };
