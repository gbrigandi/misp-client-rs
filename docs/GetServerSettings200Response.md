# GetServerSettings200Response

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**version** | Option<[**models::GetServerSettings200ResponseVersion**](getServerSettings_200_response_version.md)> |  | [optional]
**php_settings** | Option<[**models::GetServerSettings200ResponsePhpSettings**](getServerSettings_200_response_phpSettings.md)> |  | [optional]
**gpg_status** | Option<**String**> |  | [optional]
**proxy_status** | Option<**String**> |  | [optional]
**zmq_status** | Option<**i32**> |  | [optional]
**stix** | Option<[**models::GetServerSettings200ResponseStix**](getServerSettings_200_response_stix.md)> |  | [optional]
**module_status** | Option<[**models::GetServerSettings200ResponseModuleStatus**](getServerSettings_200_response_moduleStatus.md)> |  | [optional]
**writeable_dirs** | Option<[**serde_json::Value**](.md)> |  | [optional]
**writeable_files** | Option<[**serde_json::Value**](.md)> |  | [optional]
**readable_files** | Option<[**serde_json::Value**](.md)> |  | [optional]
**db_diagnostics** | Option<[**serde_json::Value**](.md)> |  | [optional]
**db_schema_diagnostics** | Option<[**models::GetServerSettings200ResponseDbSchemaDiagnostics**](getServerSettings_200_response_dbSchemaDiagnostics.md)> |  | [optional]
**redis_info** | Option<[**serde_json::Value**](.md)> |  | [optional]
**final_settings** | Option<[**Vec<models::MispSetting>**](MispSetting.md)> |  | [optional]
**extensions** | Option<[**models::GetServerSettings200ResponseExtensions**](getServerSettings_200_response_extensions.md)> |  | [optional]
**workers** | Option<[**models::GetServerSettings200ResponseWorkers**](getServerSettings_200_response_workers.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


