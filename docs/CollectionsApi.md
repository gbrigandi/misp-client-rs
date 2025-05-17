# \CollectionsApi

All URIs are relative to *https://misp.local*

Method | HTTP request | Description
------------- | ------------- | -------------
[**add_collection**](CollectionsApi.md#add_collection) | **POST** /collections/add | Add a collection
[**delete_collection**](CollectionsApi.md#delete_collection) | **DELETE** /collections/delete/{collectionId} | Delete a collection
[**edit_collection**](CollectionsApi.md#edit_collection) | **POST** /collections/edit/{collectionId} | Edit a collection
[**get_collection_by_id**](CollectionsApi.md#get_collection_by_id) | **GET** /collections/view/{collectionId} | View a collection by ID
[**get_collections**](CollectionsApi.md#get_collections) | **GET** /collections/index/{filter} | Get a list of collections with optional filtering



## add_collection

> models::AddCollection200Response add_collection(add_collection_request)
Add a collection

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**add_collection_request** | [**AddCollectionRequest**](AddCollectionRequest.md) |  | [required] |

### Return type

[**models::AddCollection200Response**](addCollection_200_response.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_collection

> models::DeleteCollection200Response delete_collection(collection_id)
Delete a collection

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**collection_id** | **String** | UUID or numeric ID of the collection | [required] |

### Return type

[**models::DeleteCollection200Response**](deleteCollection_200_response.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## edit_collection

> models::AddCollection200Response edit_collection(collection_id, base_collection)
Edit a collection

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**collection_id** | **String** | UUID or numeric ID of the collection | [required] |
**base_collection** | [**BaseCollection**](BaseCollection.md) |  | [required] |

### Return type

[**models::AddCollection200Response**](addCollection_200_response.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_collection_by_id

> models::GetCollectionById200Response get_collection_by_id(collection_id)
View a collection by ID

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**collection_id** | **String** | UUID or numeric ID of the collection | [required] |

### Return type

[**models::GetCollectionById200Response**](getCollectionById_200_response.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_collections

> Vec<models::GetCollections200ResponseInner> get_collections(filter, get_collections_request)
Get a list of collections with optional filtering

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**filter** | **String** | Optional filter for collections | [required] |
**get_collections_request** | Option<[**GetCollectionsRequest**](GetCollectionsRequest.md)> |  |  |

### Return type

[**Vec<models::GetCollections200ResponseInner>**](getCollections_200_response_inner.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

