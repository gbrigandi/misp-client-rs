# \ObjectsApi

All URIs are relative to *https://misp.local*

Method | HTTP request | Description
------------- | ------------- | -------------
[**add_object**](ObjectsApi.md#add_object) | **POST** /objects/add/{eventId}/{objectTemplateId} | Add an object to an event
[**delete_object**](ObjectsApi.md#delete_object) | **DELETE** /objects/delete/{objectId}/{hardDelete} | Delete object
[**get_object_by_id**](ObjectsApi.md#get_object_by_id) | **GET** /objects/view/{objectId} | Get object by ID
[**rest_search_objects**](ObjectsApi.md#rest_search_objects) | **POST** /objects/restsearch | [restSearch] Get a filtered and paginated list of objects



## add_object

> models::AddObject200Response add_object(event_id, object_template_id, add_object_request)
Add an object to an event

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**event_id** | **String** | UUID or numeric ID of the event | [required] |
**object_template_id** | **String** | UUID or numeric ID of the object template | [required] |
**add_object_request** | Option<[**AddObjectRequest**](AddObjectRequest.md)> |  |  |

### Return type

[**models::AddObject200Response**](addObject_200_response.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_object

> models::DeleteObject200Response delete_object(object_id, hard_delete)
Delete object

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**object_id** | **String** | UUID or numeric ID of the object | [required] |
**hard_delete** | **String** | `1` for hard delete the entity, `0` for soft deletion. | [required] |

### Return type

[**models::DeleteObject200Response**](deleteObject_200_response.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_object_by_id

> models::GetObjectById200Response get_object_by_id(object_id)
Get object by ID

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**object_id** | **String** | UUID or numeric ID of the object | [required] |

### Return type

[**models::GetObjectById200Response**](getObjectById_200_response.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## rest_search_objects

> models::RestSearchObjects200Response rest_search_objects(object_rest_search_filter)
[restSearch] Get a filtered and paginated list of objects

**This is the recommended endpoint for searching objects.** 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**object_rest_search_filter** | [**ObjectRestSearchFilter**](ObjectRestSearchFilter.md) |  | [required] |

### Return type

[**models::RestSearchObjects200Response**](restSearchObjects_200_response.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

