//! Tests for the monitoring module

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::{Duration, Instant};
    use std::thread;

    #[test]
    fn test_performance_config_default() {
        let config = PerformanceConfig::default();
        assert_eq!(config.startup_threshold_ms, 1000);
        assert_eq!(config.database_threshold_ms, 50);
        assert_eq!(config.ipc_threshold_ms, 100);
        assert_eq!(config.ai_request_threshold_ms, 2000);
    }

    #[test]
    fn test_performance_config_development() {
        let config = PerformanceConfig::development();
        assert_eq!(config.startup_threshold_ms, 3000);
        assert_eq!(config.database_threshold_ms, 200);
        assert_eq!(config.ipc_threshold_ms, 300);
        assert_eq!(config.ai_request_threshold_ms, 5000);
    }

    #[test]
    fn test_performance_config_production() {
        let config = PerformanceConfig::production();
        assert_eq!(config.startup_threshold_ms, 800);
        assert_eq!(config.database_threshold_ms, 30);
        assert_eq!(config.ipc_threshold_ms, 80);
        assert_eq!(config.ai_request_threshold_ms, 1500);
    }

    #[test]
    fn test_performance_config_builder() {
        let config = PerformanceConfig::builder()
            .startup_threshold_ms(500)
            .database_threshold_ms(25)
            .ipc_threshold_ms(75)
            .ai_request_threshold_ms(1000)
            .build();
        
        assert_eq!(config.startup_threshold_ms, 500);
        assert_eq!(config.database_threshold_ms, 25);
        assert_eq!(config.ipc_threshold_ms, 75);
        assert_eq!(config.ai_request_threshold_ms, 1000);
    }

    #[test]
    fn test_startup_tracking() {
        let start_time = PerformanceMonitor::start_startup_tracking();
        thread::sleep(Duration::from_millis(10)); // Simulate some work
        PerformanceMonitor::finish_startup_tracking(start_time, None);
        // No assertion needed, just checking it doesn't panic
    }

    #[test]
    fn test_database_operation_success() {
        let result = PerformanceMonitor::track_database_operation(
            "test_operation",
            || Ok::<_, Box<dyn std::error::Error>>("success"),
            None
        );
        
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "success");
    }

    #[test]
    fn test_database_operation_error() {
        let result: Result<(), _> = PerformanceMonitor::track_database_operation(
            "test_operation",
            || Err::<(), _>("test error".into()),
            None
        );
        
        assert!(result.is_err());
        match result {
            Err(MonitoringError::Operation(e)) => {
                let error_msg = format!("{}", e);
                assert!(error_msg.contains("test error"));
            },
            _ => panic!("Expected Operation error"),
        }
    }

    #[test]
    fn test_ipc_command_success() {
        let result = PerformanceMonitor::track_ipc_command(
            "test_command",
            || Ok::<_, String>("success"),
            None
        );
        
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "success");
    }

    #[test]
    fn test_ipc_command_error() {
        let result: Result<(), _> = PerformanceMonitor::track_ipc_command(
            "test_command",
            || Err::<(), _>("test error"),
            None
        );
        
        assert!(result.is_err());
        match result {
            Err(MonitoringError::Operation(e)) => {
                let error_msg = format!("{}", e);
                assert_eq!(error_msg, "test error");
            },
            _ => panic!("Expected Operation error"),
        }
    }

    #[test]
    fn test_ai_request_success() {
        PerformanceMonitor::track_ai_request(
            "test_model",
            Some(100),
            Ok::<(), String>(())
        );
        // No assertion needed, just checking it doesn't panic
    }

    #[test]
    fn test_ai_request_error() {
        PerformanceMonitor::track_ai_request(
            "test_model",
            Some(100),
            Err::<(), _>("test error")
        );
        // No assertion needed, just checking it doesn't panic
    }

    #[test]
    fn test_scoped_transaction() {
        let mut transaction = scoped_transaction("test", "test_op");
        // No assertion needed, just checking it doesn't panic
        transaction.finish();
    }

    #[test]
    fn test_scoped_transaction_drop() {
        {
            let _transaction = scoped_transaction("test", "test_op");
            // Transaction should be finished when it goes out of scope
        }
        // No assertion needed, just checking it doesn't panic
    }
}
