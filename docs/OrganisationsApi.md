# \OrganisationsApi

All URIs are relative to *https://misp.local*

Method | HTTP request | Description
------------- | ------------- | -------------
[**add_organisation**](OrganisationsApi.md#add_organisation) | **POST** /admin/organisations/add | Add organisation
[**delete_organisation**](OrganisationsApi.md#delete_organisation) | **DELETE** /admin/organisations/delete/{organisationId} | Delete organisation
[**edit_organisation**](OrganisationsApi.md#edit_organisation) | **PUT** /admin/organisations/edit/{organisationId} | Edit organisation
[**get_organisation_by_id**](OrganisationsApi.md#get_organisation_by_id) | **GET** /organisations/view/{organisationId} | Get organisation by ID
[**get_organisations**](OrganisationsApi.md#get_organisations) | **GET** /organisations | Get organisations



## add_organisation

> models::Organisation add_organisation(organisation_no_id)
Add organisation

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**organisation_no_id** | Option<[**OrganisationNoId**](OrganisationNoId.md)> |  |  |

### Return type

[**models::Organisation**](Organisation.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_organisation

> models::DeleteOrganisation200Response delete_organisation(organisation_id)
Delete organisation

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**organisation_id** | **String** | UUID or numeric ID of the organisation | [required] |

### Return type

[**models::DeleteOrganisation200Response**](deleteOrganisation_200_response.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## edit_organisation

> models::Organisation edit_organisation(organisation_id, edit_organisation_request)
Edit organisation

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**organisation_id** | **String** | UUID or numeric ID of the organisation | [required] |
**edit_organisation_request** | Option<[**EditOrganisationRequest**](EditOrganisationRequest.md)> |  |  |

### Return type

[**models::Organisation**](Organisation.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_organisation_by_id

> models::Organisation get_organisation_by_id(organisation_id)
Get organisation by ID

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**organisation_id** | **String** | UUID or numeric ID of the organisation | [required] |

### Return type

[**models::Organisation**](Organisation.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_organisations

> Vec<models::OrganisationListItem> get_organisations()
Get organisations

### Parameters

This endpoint does not need any parameter.

### Return type

[**Vec<models::OrganisationListItem>**](OrganisationListItem.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

