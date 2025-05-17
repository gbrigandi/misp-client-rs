# FeedNoId

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**name** | Option<**String**> |  | [optional]
**provider** | Option<**String**> |  | [optional]
**url** | Option<**String**> |  | [optional]
**rules** | Option<**String**> | Stringified JSON filter rules. | [optional]
**enabled** | Option<**bool**> |  | [optional]
**distribution** | Option<[**models::DistributionLevelId**](DistributionLevelId.md)> |  | [optional]
**sharing_group_id** | Option<**String**> |  | [optional]
**tag_id** | Option<**String**> |  | [optional]
**default** | Option<**bool**> |  | [optional]
**source_format** | Option<[**models::FeedSourceFormat**](FeedSourceFormat.md)> |  | [optional]
**fixed_event** | Option<**bool**> | target_event option might be considered | [optional]
**delta_merge** | Option<**bool**> | Merge attributes (only add new attribute, remove revoked attributes) | [optional]
**event_id** | Option<**String**> |  | [optional]
**publish** | Option<**bool**> |  | [optional][default to false]
**override_ids** | Option<**bool**> | The IDS flags will be set to Off for this feed | [optional]
**settings** | Option<**String**> |  | [optional]
**input_source** | Option<[**models::FeedInputSource**](FeedInputSource.md)> |  | [optional]
**delete_local_file** | Option<**bool**> | The IDS flags will be set to Off for this feed | [optional]
**lookup_visible** | Option<**bool**> | The lookup will not be visible in the feed correlation | [optional]
**headers** | Option<**String**> | Headers to be passed with the requests. All separated by   | [optional]
**caching_enabled** | Option<**bool**> | The feed is cached | [optional]
**force_to_ids** | Option<**bool**> | The IDS flags will be set to On for this feed | [optional]
**orgc_id** | Option<**String**> |  | [optional]
**cache_timestamp** | Option<[**models::FeedCacheTimestamp**](FeedCacheTimestamp.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


