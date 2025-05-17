# GetServerSettings200ResponseDbSchemaDiagnostics

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**data_source** | Option<**String**> |  | [optional]
**actual_db_version** | Option<**String**> |  | [optional]
**checked_table_column** | Option<**Vec<String>**> |  | [optional]
**diagnostic** | Option<[**serde_json::Value**](.md)> |  | [optional]
**diagnostic_index** | Option<[**models::GetServerSettings200ResponseDbSchemaDiagnosticsDiagnosticIndex**](getServerSettings_200_response_dbSchemaDiagnostics_diagnostic_index.md)> |  | [optional]
**expected_db_version** | Option<**String**> |  | [optional]
**error** | Option<**String**> |  | [optional]
**update_locked** | Option<**bool**> |  | [optional]
**remaining_lock_time** | Option<**f64**> |  | [optional]
**update_fail_number_reached** | Option<**bool**> |  | [optional]
**indexes** | Option<[**serde_json::Value**](.md)> |  | [optional]
**column_per_table** | Option<[**serde_json::Value**](.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


