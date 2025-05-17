# \LogsApi

All URIs are relative to *https://misp.local*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_logs**](LogsApi.md#get_logs) | **POST** /admin/logs | Get instance logs



## get_logs

> Vec<models::GetLogs200ResponseInner> get_logs(get_logs_request)
Get instance logs

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**get_logs_request** | Option<[**GetLogsRequest**](GetLogsRequest.md)> |  |  |

### Return type

[**Vec<models::GetLogs200ResponseInner>**](getLogs_200_response_inner.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

