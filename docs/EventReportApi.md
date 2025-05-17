# \EventReportApi

All URIs are relative to *https://misp.local*

Method | HTTP request | Description
------------- | ------------- | -------------
[**add_event_report**](EventReportApi.md#add_event_report) | **POST** /eventReports/add/{eventId} | Add Event Report
[**delete_event_report**](EventReportApi.md#delete_event_report) | **POST** /eventReports/delete/{eventReportId} | Delete Event Report
[**edit_event_report**](EventReportApi.md#edit_event_report) | **POST** /eventReports/edit/{eventReportId} | Edit Event Report
[**hard_delete_event_report**](EventReportApi.md#hard_delete_event_report) | **POST** /eventReports/delete/{eventReportId}/{hardDelete} | Hard Delete Event Report
[**import_from_url_event_report**](EventReportApi.md#import_from_url_event_report) | **POST** /eventReports/importReportFromUrl/{eventId} | Import Report From URL
[**index_event_report**](EventReportApi.md#index_event_report) | **GET** /eventReports/index | Get event reports
[**restore_event_report**](EventReportApi.md#restore_event_report) | **POST** /eventReports/restore/{eventReportId} | Restore Event Report
[**view_event_report**](EventReportApi.md#view_event_report) | **GET** /eventReports/view/{eventReportId} | Get event report by ID



## add_event_report

> models::ViewEventReport200Response add_event_report(event_id, event_report_no_id)
Add Event Report

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**event_id** | **String** | UUID or numeric ID of the event | [required] |
**event_report_no_id** | [**EventReportNoId**](EventReportNoId.md) |  | [required] |

### Return type

[**models::ViewEventReport200Response**](viewEventReport_200_response.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_event_report

> models::DeleteEventReport200Response delete_event_report(event_report_id)
Delete Event Report

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**event_report_id** | **String** | UUID or numeric ID of the event report | [required] |

### Return type

[**models::DeleteEventReport200Response**](deleteEventReport_200_response.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## edit_event_report

> models::ViewEventReport200Response edit_event_report(event_report_id)
Edit Event Report

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**event_report_id** | **String** | UUID or numeric ID of the event report | [required] |

### Return type

[**models::ViewEventReport200Response**](viewEventReport_200_response.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## hard_delete_event_report

> models::DeleteEventReport200Response hard_delete_event_report(event_report_id, hard_delete)
Hard Delete Event Report

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**event_report_id** | **String** | UUID or numeric ID of the event report | [required] |
**hard_delete** | **String** | `1` for hard delete the entity, `0` for soft deletion. | [required] |

### Return type

[**models::DeleteEventReport200Response**](deleteEventReport_200_response.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## import_from_url_event_report

> models::ImportFromUrlEventReport200Response import_from_url_event_report(event_id, import_from_url_event_report_request)
Import Report From URL

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**event_id** | **String** | UUID or numeric ID of the event | [required] |
**import_from_url_event_report_request** | [**ImportFromUrlEventReportRequest**](ImportFromUrlEventReportRequest.md) |  | [required] |

### Return type

[**models::ImportFromUrlEventReport200Response**](importFromURLEventReport_200_response.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## index_event_report

> Vec<models::ViewEventReport200Response> index_event_report()
Get event reports

### Parameters

This endpoint does not need any parameter.

### Return type

[**Vec<models::ViewEventReport200Response>**](viewEventReport_200_response.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## restore_event_report

> models::RestoreEventReport200Response restore_event_report(event_report_id)
Restore Event Report

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**event_report_id** | **String** | UUID or numeric ID of the event report | [required] |

### Return type

[**models::RestoreEventReport200Response**](restoreEventReport_200_response.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## view_event_report

> models::ViewEventReport200Response view_event_report(event_report_id)
Get event report by ID

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**event_report_id** | **String** | UUID or numeric ID of the event report | [required] |

### Return type

[**models::ViewEventReport200Response**](viewEventReport_200_response.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

