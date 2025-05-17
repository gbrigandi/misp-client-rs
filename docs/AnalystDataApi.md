# \AnalystDataApi

All URIs are relative to *https://misp.local*

Method | HTTP request | Description
------------- | ------------- | -------------
[**add_analyst_data**](AnalystDataApi.md#add_analyst_data) | **POST** /analystData/add/{analystType}/{analystObjectUUID}/{analystObjectType} | Add analyst data
[**delete_analyst_data**](AnalystDataApi.md#delete_analyst_data) | **DELETE** /analystData/delete/{analystType}/{analystDataID} | Delete Analyst data
[**edit_analyst_data**](AnalystDataApi.md#edit_analyst_data) | **POST** /analystData/edit/{analystType}/{analystDataID} | Edit analyst data
[**get_analyst_data_by_id**](AnalystDataApi.md#get_analyst_data_by_id) | **GET** /analystData/view/{analystType}/{analystDataID} | Get Analyst Data by ID
[**index_analyst_data**](AnalystDataApi.md#index_analyst_data) | **GET** /analystData/index/{analystType} | List Analyst data
[**index_minimal_analyst_data**](AnalystDataApi.md#index_minimal_analyst_data) | **GET** /analystData/indexMinimal | List minimal Analyst data



## add_analyst_data

> models::AddAnalystDataRequest add_analyst_data(analyst_type, analyst_object_uuid, analyst_object_type, add_analyst_data_request)
Add analyst data

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**analyst_type** | [**AnalystDataType**](.md) | Type of the analyst data. | [required] |
**analyst_object_uuid** | **uuid::Uuid** | Object UUID that has an analyst data | [required] |
**analyst_object_type** | [**AnalystObjectType**](.md) | Object type that has an analyst data | [required] |
**add_analyst_data_request** | [**AddAnalystDataRequest**](AddAnalystDataRequest.md) |  | [required] |

### Return type

[**models::AddAnalystDataRequest**](addAnalystData_request.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_analyst_data

> models::DeleteAnalystData200Response delete_analyst_data(analyst_type, analyst_data_id)
Delete Analyst data

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**analyst_type** | [**AnalystDataType**](.md) | Type of the analyst data. | [required] |
**analyst_data_id** | **String** | UUID or numeric ID of the Analyst data | [required] |

### Return type

[**models::DeleteAnalystData200Response**](deleteAnalystData_200_response.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## edit_analyst_data

> models::AddAnalystDataRequest edit_analyst_data(analyst_type, analyst_data_id, add_analyst_data_request)
Edit analyst data

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**analyst_type** | [**AnalystDataType**](.md) | Type of the analyst data. | [required] |
**analyst_data_id** | **String** | UUID or numeric ID of the Analyst data | [required] |
**add_analyst_data_request** | [**AddAnalystDataRequest**](AddAnalystDataRequest.md) |  | [required] |

### Return type

[**models::AddAnalystDataRequest**](addAnalystData_request.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_analyst_data_by_id

> models::AddAnalystDataRequest get_analyst_data_by_id(analyst_type, analyst_data_id)
Get Analyst Data by ID

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**analyst_type** | [**AnalystDataType**](.md) | Type of the analyst data. | [required] |
**analyst_data_id** | **String** | UUID or numeric ID of the Analyst data | [required] |

### Return type

[**models::AddAnalystDataRequest**](addAnalystData_request.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## index_analyst_data

> Vec<models::AddAnalystDataRequest> index_analyst_data(analyst_type)
List Analyst data

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**analyst_type** | [**AnalystDataType**](.md) | Type of the analyst data. | [required] |

### Return type

[**Vec<models::AddAnalystDataRequest>**](addAnalystData_request.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## index_minimal_analyst_data

> models::IndexMinimalAnalystData200Response index_minimal_analyst_data(index_minimal_analyst_data_request)
List minimal Analyst data

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**index_minimal_analyst_data_request** | Option<[**IndexMinimalAnalystDataRequest**](IndexMinimalAnalystDataRequest.md)> |  |  |

### Return type

[**models::IndexMinimalAnalystData200Response**](indexMinimalAnalystData_200_response.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

