# \GalaxiesApi

All URIs are relative to *https://misp.local*

Method | HTTP request | Description
------------- | ------------- | -------------
[**attach_galaxy_cluster**](GalaxiesApi.md#attach_galaxy_cluster) | **POST** /galaxies/attachCluster/{attachTargetId}/{attachTargetType}/local:{local} | Attach the galaxy cluster tag a given entity
[**delete_galaxy**](GalaxiesApi.md#delete_galaxy) | **DELETE** /galaxies/delete/{galaxyId} | Delete a galaxy
[**export_galaxy_clusters**](GalaxiesApi.md#export_galaxy_clusters) | **POST** /galaxies/export/{galaxyId} | Export galaxy clusters
[**get_galaxies**](GalaxiesApi.md#get_galaxies) | **GET** /galaxies | Get galaxies
[**get_galaxy_by_id**](GalaxiesApi.md#get_galaxy_by_id) | **GET** /galaxies/view/{galaxyId} | Get galaxy by ID
[**import_galaxy_cluster**](GalaxiesApi.md#import_galaxy_cluster) | **POST** /galaxies/import | Import a galaxy cluster
[**search_galaxies**](GalaxiesApi.md#search_galaxies) | **POST** /galaxies | Search galaxies
[**update_galaxies**](GalaxiesApi.md#update_galaxies) | **POST** /galaxies/update | Force update the galaxies with the galaxy json definitions



## attach_galaxy_cluster

> models::AttachGalaxyCluster200Response attach_galaxy_cluster(attach_target_id, attach_target_type, local, attach_galaxy_cluster_request)
Attach the galaxy cluster tag a given entity

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**attach_target_id** | **String** | UUID or numeric ID of the target entity (Event, Attribute or TagCollection) | [required] |
**attach_target_type** | **String** | Type of the target entity to attach to the galaxy cluster. | [required] |
**local** | **bool** | Whether the object should be attached locally or not to the target | [required] |
**attach_galaxy_cluster_request** | Option<[**AttachGalaxyClusterRequest**](AttachGalaxyClusterRequest.md)> |  |  |

### Return type

[**models::AttachGalaxyCluster200Response**](attachGalaxyCluster_200_response.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_galaxy

> models::DeleteGalaxy200Response delete_galaxy(galaxy_id)
Delete a galaxy

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**galaxy_id** | **String** | UUID or numeric ID of the galaxy | [required] |

### Return type

[**models::DeleteGalaxy200Response**](deleteGalaxy_200_response.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## export_galaxy_clusters

> models::ExportGalaxyClusters200Response export_galaxy_clusters(galaxy_id, export_galaxy_clusters_request)
Export galaxy clusters

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**galaxy_id** | **String** | UUID or numeric ID of the galaxy | [required] |
**export_galaxy_clusters_request** | Option<[**ExportGalaxyClustersRequest**](ExportGalaxyClustersRequest.md)> |  |  |

### Return type

[**models::ExportGalaxyClusters200Response**](exportGalaxyClusters_200_response.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_galaxies

> Vec<models::GetGalaxies200ResponseInner> get_galaxies()
Get galaxies

### Parameters

This endpoint does not need any parameter.

### Return type

[**Vec<models::GetGalaxies200ResponseInner>**](getGalaxies_200_response_inner.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_galaxy_by_id

> models::ExtendedGalaxy get_galaxy_by_id(galaxy_id)
Get galaxy by ID

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**galaxy_id** | **String** | UUID or numeric ID of the galaxy | [required] |

### Return type

[**models::ExtendedGalaxy**](ExtendedGalaxy.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## import_galaxy_cluster

> models::ImportGalaxyCluster200Response import_galaxy_cluster(import_galaxy_cluster_item)
Import a galaxy cluster

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**import_galaxy_cluster_item** | [**Vec<models::ImportGalaxyClusterItem>**](ImportGalaxyClusterItem.md) |  | [required] |

### Return type

[**models::ImportGalaxyCluster200Response**](importGalaxyCluster_200_response.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## search_galaxies

> Vec<models::GetGalaxies200ResponseInner> search_galaxies(search_galaxies_request)
Search galaxies

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**search_galaxies_request** | [**SearchGalaxiesRequest**](SearchGalaxiesRequest.md) |  | [required] |

### Return type

[**Vec<models::GetGalaxies200ResponseInner>**](getGalaxies_200_response_inner.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_galaxies

> models::UpdateGalaxies200Response update_galaxies()
Force update the galaxies with the galaxy json definitions

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::UpdateGalaxies200Response**](updateGalaxies_200_response.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

