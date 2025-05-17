# \TaxonomiesApi

All URIs are relative to *https://misp.local*

Method | HTTP request | Description
------------- | ------------- | -------------
[**disable_taxonomy**](TaxonomiesApi.md#disable_taxonomy) | **POST** /taxonomies/disable/{taxonomyId} | Disable taxonomy
[**enable_taxonomy**](TaxonomiesApi.md#enable_taxonomy) | **POST** /taxonomies/enable/{taxonomyId} | Enable taxonomy
[**export_taxonomy**](TaxonomiesApi.md#export_taxonomy) | **GET** /taxonomies/export/{taxonomyId} | Export taxonomy.
[**get_taxonomies**](TaxonomiesApi.md#get_taxonomies) | **GET** /taxonomies | Get taxonomies
[**get_taxonomy_by_id**](TaxonomiesApi.md#get_taxonomy_by_id) | **GET** /taxonomies/view/{taxonomyId} | Get a taxonomy by ID
[**get_taxonomy_tags**](TaxonomiesApi.md#get_taxonomy_tags) | **GET** /taxonomies/taxonomy_tags/{taxonomyId} | Get a taxonomy extended with tags used in events and attributes.
[**update_taxonomies**](TaxonomiesApi.md#update_taxonomies) | **POST** /taxonomies/update | Update taxonomies



## disable_taxonomy

> models::DisableTaxonomy200Response disable_taxonomy(taxonomy_id)
Disable taxonomy

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**taxonomy_id** | **String** | Numeric ID of the taxonomy | [required] |

### Return type

[**models::DisableTaxonomy200Response**](disableTaxonomy_200_response.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## enable_taxonomy

> models::EnableTaxonomy200Response enable_taxonomy(taxonomy_id)
Enable taxonomy

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**taxonomy_id** | **String** | Numeric ID of the taxonomy | [required] |

### Return type

[**models::EnableTaxonomy200Response**](enableTaxonomy_200_response.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## export_taxonomy

> models::ExportTaxonomy200Response export_taxonomy(taxonomy_id)
Export taxonomy.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**taxonomy_id** | **String** | Numeric ID of the taxonomy | [required] |

### Return type

[**models::ExportTaxonomy200Response**](exportTaxonomy_200_response.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_taxonomies

> Vec<models::GetTaxonomies200ResponseInner> get_taxonomies()
Get taxonomies

### Parameters

This endpoint does not need any parameter.

### Return type

[**Vec<models::GetTaxonomies200ResponseInner>**](getTaxonomies_200_response_inner.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_taxonomy_by_id

> models::GetTaxonomyById200Response get_taxonomy_by_id(taxonomy_id)
Get a taxonomy by ID

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**taxonomy_id** | **String** | Numeric ID of the taxonomy | [required] |

### Return type

[**models::GetTaxonomyById200Response**](getTaxonomyById_200_response.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_taxonomy_tags

> models::GetTaxonomyTags200Response get_taxonomy_tags(taxonomy_id)
Get a taxonomy extended with tags used in events and attributes.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**taxonomy_id** | **String** | Numeric ID of the taxonomy | [required] |

### Return type

[**models::GetTaxonomyTags200Response**](getTaxonomyTags_200_response.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_taxonomies

> models::UpdateTaxonomies200Response update_taxonomies()
Update taxonomies

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::UpdateTaxonomies200Response**](updateTaxonomies_200_response.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

