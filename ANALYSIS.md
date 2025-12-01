# Codebase Analysis & Recommendations

## Executive Summary
This is a Next.js-like API route handler built in Rust using MetaCall (not deno_core as mentioned in README) to execute JavaScript, TypeScript, and Rust code. The project has a solid foundation but contains several critical bugs, architectural issues, and missing features.

---

## 🔴 Critical Issues (Must Fix)

### 1. **Cargo.toml Configuration Errors**
- **Line 4**: `edition = "2024"` - This edition doesn't exist. Should be `"2021"` (latest stable).
- **Line 12**: `api_dir = { path = "./api" }` - Invalid dependency syntax. This should be removed or properly configured as a workspace member (which it already is on line 20).

### 2. **JavaScript Runtime Bug - Request Object Not Injected**
- **Location**: `src/runtimes/js_runtime.rs:38`
- **Issue**: `const request = {};` should be `const request = {request_json};` (actually should parse the JSON)
- **Impact**: Request data (headers, body, query, etc.) is not being passed to handler functions
- **Fix**: Should be `const request = JSON.parse('{request_json}');`

### 3. **TypeScript Runtime - Duplicate/Unused Code**
- **Location**: `src/runtimes/ts_runtime.rs`
- **Issue**: File is identical to `js_runtime.rs` but never imported/used. TypeScript uses `execute_js_file` from `js_runtime.rs` instead.
- **Impact**: Code duplication, confusion, maintenance burden
- **Fix**: Either remove `ts_runtime.rs` or consolidate the logic

### 4. **Rust Runtime - Non-Functional**
- **Location**: `src/runtimes/rs_runtime.rs`
- **Issues**:
  - Line 23: `load::from_memory(load::Tag::Rust, &code, None)` - Handle is created but not used
  - No actual function call to execute the Rust code
  - Always returns hardcoded 200 status with empty body
  - Request data is never passed to Rust handlers
- **Impact**: Rust API routes don't work at all

### 5. **Error Handling - Panic Risks**
- **Locations**: Multiple places using `unwrap()` and `expect()`
  - `src/main.rs:113` - `unwrap()` on TcpListener bind
  - `src/main.rs:115` - `unwrap()` on serve
  - `src/runtimes/js_runtime.rs:83` - `expect()` on JSON parsing
  - `src/main.rs:151, 162, 172` - `unwrap()` on JSON serialization
- **Impact**: Server can crash unexpectedly
- **Fix**: Use proper error handling with `?` operator or match statements

### 6. **Request Body Extraction Issue**
- **Location**: `src/main.rs:123`
- **Issue**: Using `body: String` extractor which may not work correctly for all content types
- **Impact**: Request bodies might not be read properly
- **Fix**: Use `axum::extract::Body` or `axum::body::Body` and convert to string

---

## 🟡 Important Issues (Should Fix)

### 7. **Debug Print Statements**
- **Location**: `src/runtimes/js_runtime.rs:60, 81`
- **Issue**: `print!()` statements left in production code
- **Impact**: Unnecessary console output, potential performance issues
- **Fix**: Remove or use proper logging (e.g., `tracing` or `log` crate)

### 8. **Missing Response Headers Support**
- **Location**: `src/runtimes/js_runtime.rs` - `JsResponse` struct
- **Issue**: No way for handlers to set custom HTTP headers
- **Impact**: Can't set CORS, content-type, caching headers, etc.
- **Fix**: Add `headers: HashMap<String, String>` to `JsResponse`

### 9. **No Content-Type Header Set**
- **Location**: `src/main.rs:151, 162, 172`
- **Issue**: Responses don't set `Content-Type: application/json` header
- **Impact**: Clients may not parse JSON correctly
- **Fix**: Add header in response

### 10. **Incomplete Route Scanning**
- **Location**: `src/main.rs:184-216` - `scan_api_dir()`
- **Issues**:
  - Only scans top-level files, not subdirectories
  - No support for nested routes like `api/users/profile.js`
  - No support for dynamic routes like `api/user/[id].js`
- **Impact**: Limited routing capabilities
- **Fix**: Implement recursive directory scanning and dynamic route matching

### 11. **No Request Body Parsing for Non-JSON**
- **Location**: `src/main.rs:143`
- **Issue**: Body is passed as raw string, but handlers expect JSON
- **Impact**: Can't handle form data, text/plain, etc.
- **Fix**: Parse based on Content-Type header

### 12. **Missing README Update**
- **Location**: `README.md:1`
- **Issue**: Says using `deno_core` but actually using `metacall`
- **Impact**: Misleading documentation
- **Fix**: Update README to reflect actual implementation

---

## 🟢 Feature Gaps (From TODO List)

### 13. **GET Route Caching**
- **Status**: Not implemented
- **Priority**: Medium
- **Suggestion**: Use `moka` or `cacache` crate for in-memory caching with TTL

### 14. **Dynamic Routing**
- **Status**: Not implemented
- **Priority**: High (in TODO)
- **Suggestion**: 
  - Support `[id].js` or `[id].ts` file naming
  - Use regex or `matchit` crate for route matching
  - Extract params and populate `JsRequest.params`

### 15. **Rust Language Support**
- **Status**: Partially implemented (broken)
- **Priority**: Medium
- **Suggestion**: Fix `rs_runtime.rs` to actually execute Rust code via MetaCall

---

## 🔵 Code Quality Improvements

### 16. **Code Duplication**
- TypeScript and JavaScript handlers use identical code paths
- Consider creating a unified handler function

### 17. **Magic Strings**
- Route paths like `/api/{}` are hardcoded
- Consider making configurable

### 18. **No Request Validation**
- No validation of request structure before passing to handlers
- Consider adding request validation layer

### 19. **No Logging Framework**
- Using `println!` instead of proper logging
- Consider `tracing` or `log` crate

### 20. **Missing Tests**
- No unit tests or integration tests
- Consider adding tests for route scanning, runtime execution

### 21. **No Error Types**
- Using `Box<dyn std::error::Error>` everywhere
- Consider custom error types with `thiserror` or `anyhow`

### 22. **TypeScript Type Safety**
- TypeScript handlers don't have proper type definitions
- Consider generating TypeScript types for `JsRequest` and `JsResponse`

---

## 📋 Recommended Implementation Order

### Phase 1: Critical Fixes (Do First)
1. Fix Cargo.toml edition and dependency issues
2. Fix request object injection in JavaScript runtime
3. Fix Rust runtime to actually execute code
4. Replace `unwrap()`/`expect()` with proper error handling
5. Fix request body extraction

### Phase 2: Important Fixes
6. Remove debug print statements
7. Add response headers support
8. Set Content-Type header on responses
9. Update README to reflect MetaCall usage

### Phase 3: Features
10. Implement dynamic routing (`[id]` support)
11. Implement GET route caching
12. Add recursive directory scanning for nested routes
13. Add proper logging framework

### Phase 4: Polish
14. Add tests
15. Add custom error types
16. Improve code organization
17. Add TypeScript type definitions

---

## 🛠️ Quick Wins (Easy Fixes)

1. **Fix Cargo.toml edition** - Change `"2024"` to `"2021"`
2. **Remove debug prints** - Delete `print!()` statements
3. **Add Content-Type header** - One line fix in response
4. **Update README** - Change deno_core to metacall
5. **Remove unused ts_runtime.rs** - Or actually use it

---

## 📝 Additional Notes

- The project structure is good - separation of concerns with `runtimes/` module
- The API file examples (`hello.js`, `users.js`) are well-written
- The route scanning logic is clean but needs extension for nested routes
- Consider using `axum::extract::Path` for dynamic route parameters
- Consider using `tower-http` for CORS, compression, etc.

---

## Questions to Consider

1. **Performance**: Should we cache compiled MetaCall handles per file?
2. **Hot Reload**: Should we watch for file changes and reload routes?
3. **Middleware**: Should we support middleware functions?
4. **Validation**: Should we validate handler function signatures?
5. **Async**: Should handlers support async/await in JavaScript?

