# Plugin Debugging Example

This directory contains examples and documentation for the xvim plugin debugging system. The debugging system provides comprehensive tools for diagnosing and fixing issues in plugins.

## Debug Configuration

The `debug_config.json` file demonstrates how to configure the debugging system:

```json
{
  "enabled": true,
  "log_level": "Debug",
  "log_file": "plugins/logs/my_plugin.log",
  "enable_tracing": true,
  "enable_snapshots": true,
  "max_log_size": 10485760,
  "max_log_files": 5
}
```

### Configuration Options

- `enabled`: Enable or disable debugging
- `log_level`: Minimum log level to record (Trace, Debug, Info, Warn, Error, Fatal)
- `log_file`: Path to the log file (relative to the xvim directory)
- `enable_tracing`: Enable performance tracing
- `enable_snapshots`: Enable state snapshots
- `max_log_size`: Maximum log file size in bytes before rotation (10MB in the example)
- `max_log_files`: Maximum number of rotated log files to keep

## Using the Debugging API

### Logging

The debugging system provides a multi-level logging system:

```rust
// Log an informational message
editor_log(LogLevel::Info, "Plugin initialized", None);

// Log a debug message with additional data
editor_log(LogLevel::Debug, "Processing data", Some("items: 42"));

// Log an error
editor_log(LogLevel::Error, "Failed to load file", Some("error: File not found"));
```

### Performance Tracing

Performance tracing allows you to measure the execution time of operations:

```rust
// Start a trace
let trace_id = editor_start_trace("load_data", Some("Loading user data"));

// ... perform the operation ...

// End the trace
editor_end_trace(trace_id);
```

The trace will record the start time, end time, and duration of the operation.

### State Snapshots

State snapshots capture the state of a plugin at a specific point in time:

```rust
// Create a map of variables to include in the snapshot
let mut variables = HashMap::new();
variables.insert("user_count".to_string(), "42".to_string());
variables.insert("cache_size".to_string(), "1024".to_string());

// Take a snapshot with memory usage, active task count, and variables
editor_take_snapshot(1024 * 1024, 2, variables);
```

### Exporting Debug Information

The debugging system provides tools for exporting debug information:

```rust
// Export logs to a file
editor_export_logs("logs.json");

// Export traces to a file
editor_export_traces("traces.json");

// Export snapshots to a file
editor_export_snapshots("snapshots.json");
```

## Analyzing Debug Information

The exported debug information can be analyzed using various tools:

- **Logs**: Use a JSON viewer or log analyzer to examine logs
- **Traces**: Use a performance visualization tool to analyze traces
- **Snapshots**: Use a JSON viewer to examine snapshots

## Best Practices

1. **Use Appropriate Log Levels**: Use the appropriate log level for each message
   - Trace: Very detailed information for debugging
   - Debug: Detailed information useful for debugging
   - Info: General information about the plugin's operation
   - Warn: Potential issues that don't prevent the plugin from working
   - Error: Issues that prevent a feature from working
   - Fatal: Issues that prevent the plugin from working at all

2. **Include Context in Logs**: Include relevant context in log messages to make them more useful

3. **Trace Performance-Critical Operations**: Use performance tracing for operations that might affect performance

4. **Take Snapshots at Key Points**: Take state snapshots at key points in the plugin's lifecycle

5. **Rotate Logs**: Configure log rotation to prevent logs from growing too large

6. **Export Debug Information**: Export debug information for offline analysis

## Debugging Workflow

1. **Enable Debugging**: Enable debugging in the debug configuration
2. **Set Log Level**: Set the appropriate log level for your needs
3. **Add Logging**: Add logging statements to your plugin
4. **Add Tracing**: Add performance tracing to critical operations
5. **Add Snapshots**: Add state snapshots at key points
6. **Run the Plugin**: Run the plugin and exercise the functionality
7. **Analyze Debug Information**: Export and analyze the debug information
8. **Fix Issues**: Fix any issues identified through debugging
9. **Repeat**: Repeat the process until the plugin works correctly