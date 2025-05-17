# \FeedsApi

All URIs are relative to *https://misp.local*

Method | HTTP request | Description
------------- | ------------- | -------------
[**add_feed**](FeedsApi.md#add_feed) | **POST** /feeds/add | Add a feed
[**cache_feeds**](FeedsApi.md#cache_feeds) | **POST** /feeds/cacheFeeds/{cacheFeedsScope} | Cache feeds
[**disable_feed**](FeedsApi.md#disable_feed) | **POST** /feeds/disable/{feedId} | Disable feed
[**edit_feed**](FeedsApi.md#edit_feed) | **PUT** /feeds/edit/{feedId} | Edit a feed
[**enable_feed**](FeedsApi.md#enable_feed) | **POST** /feeds/enable/{feedId} | Enable feed
[**fetch_from_all_feeds**](FeedsApi.md#fetch_from_all_feeds) | **POST** /feeds/fetchFromAllFeeds | Fetch from all feeds
[**fetch_from_feed**](FeedsApi.md#fetch_from_feed) | **POST** /feeds/fetchFromFeed/{feedId} | Fetch from feed by ID
[**get_feed_by_id**](FeedsApi.md#get_feed_by_id) | **GET** /feeds/view/{feedId} | Get a feed by ID
[**get_feeds**](FeedsApi.md#get_feeds) | **GET** /feeds | Get a list of feeds



## add_feed

> models::GetFeeds200ResponseInner add_feed(add_feed_request)
Add a feed

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**add_feed_request** | Option<[**AddFeedRequest**](AddFeedRequest.md)> |  |  |

### Return type

[**models::GetFeeds200ResponseInner**](getFeeds_200_response_inner.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## cache_feeds

> models::CacheFeeds200Response cache_feeds(cache_feeds_scope)
Cache feeds

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**cache_feeds_scope** | **String** | Cache feeds strategy | [required] |

### Return type

[**models::CacheFeeds200Response**](cacheFeeds_200_response.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## disable_feed

> models::DisableFeed200Response disable_feed(feed_id)
Disable feed

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**feed_id** | **String** | UUID or numeric ID of the feed | [required] |

### Return type

[**models::DisableFeed200Response**](disableFeed_200_response.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## edit_feed

> models::GetFeeds200ResponseInner edit_feed(feed_id, edit_feed_request)
Edit a feed

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**feed_id** | **String** | UUID or numeric ID of the feed | [required] |
**edit_feed_request** | Option<[**EditFeedRequest**](EditFeedRequest.md)> |  |  |

### Return type

[**models::GetFeeds200ResponseInner**](getFeeds_200_response_inner.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## enable_feed

> models::EnableFeed200Response enable_feed(feed_id)
Enable feed

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**feed_id** | **String** | UUID or numeric ID of the feed | [required] |

### Return type

[**models::EnableFeed200Response**](enableFeed_200_response.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## fetch_from_all_feeds

> models::FetchFromFeed200Response fetch_from_all_feeds()
Fetch from all feeds

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::FetchFromFeed200Response**](fetchFromFeed_200_response.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## fetch_from_feed

> models::FetchFromFeed200Response fetch_from_feed(feed_id)
Fetch from feed by ID

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**feed_id** | **String** | UUID or numeric ID of the feed | [required] |

### Return type

[**models::FetchFromFeed200Response**](fetchFromFeed_200_response.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_feed_by_id

> models::GetFeeds200ResponseInner get_feed_by_id(feed_id)
Get a feed by ID

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**feed_id** | **String** | UUID or numeric ID of the feed | [required] |

### Return type

[**models::GetFeeds200ResponseInner**](getFeeds_200_response_inner.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_feeds

> Vec<models::GetFeeds200ResponseInner> get_feeds()
Get a list of feeds

### Parameters

This endpoint does not need any parameter.

### Return type

[**Vec<models::GetFeeds200ResponseInner>**](getFeeds_200_response_inner.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

