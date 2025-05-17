# RestSearchEventsRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**page** | Option<**i32**> |  | [optional]
**limit** | Option<**i32**> |  | [optional]
**value** | Option<**String**> |  | [optional]
**r#type** | Option<[**models::AttributeType**](AttributeType.md)> |  | [optional]
**category** | Option<[**models::AttributeCategory**](AttributeCategory.md)> |  | [optional]
**org** | Option<[**models::RestSearchEventsRequestOrg**](restSearchEvents_request_org.md)> |  | [optional]
**tags** | Option<**Vec<String>**> |  | [optional]
**event_tags** | Option<**Vec<String>**> |  | [optional]
**searchall** | Option<**String**> | Search events by matching any tag names, event descriptions, attribute values or attribute comments | [optional]
**from** | Option<**String**> | You can use any of the valid time related filters (examples: 7d, timestamps, [14d, 7d] for ranges, etc.) | [optional]
**to** | Option<**String**> | You can use any of the valid time related filters (examples: 7d, timestamps, [14d, 7d] for ranges, etc.) | [optional]
**last** | Option<[**models::LastRestSearchFilter**](LastRestSearchFilter.md)> |  | [optional]
**eventid** | Option<**String**> |  | [optional]
**with_attachments** | Option<**bool**> | Extends the response with the base64 representation of the attachment, if there is one | [optional][default to false]
**sharinggroup** | Option<**Vec<String>**> | Sharing group ID(s), either as single string or list of IDs | [optional]
**metadata** | Option<**bool**> | Will only return the metadata of the given query scope, contained data is omitted. | [optional]
**uuid** | Option<[**uuid::Uuid**](uuid::Uuid.md)> |  | [optional]
**publish_timestamp** | Option<**String**> |  | [optional][default to 0]
**timestamp** | Option<**String**> |  | [optional][default to 0]
**published** | Option<**bool**> |  | [optional][default to false]
**enforce_warninglist** | Option<**bool**> | Should the warning list be enforced. Adds blocked field for matching attributes | [optional]
**sg_reference_only** | Option<**bool**> | Will only return the sharing group ID | [optional]
**requested_attributes** | Option<**Vec<String>**> | List of properties that will be selected in the CSV export | [optional]
**include_context** | Option<**bool**> | Adds events context fields in the CSV export | [optional]
**headerless** | Option<**bool**> | Removes header in the CSV export | [optional]
**include_warninglist_hits** | Option<**bool**> |  | [optional]
**attack_galaxy** | Option<**String**> |  | [optional]
**to_ids** | Option<**bool**> |  | [optional][default to true]
**deleted** | Option<**bool**> | Whether to include soft-deleted attributes. Default value 0. If set to 1, only deleted attributes will be returned. If set to [0,1], both deleted and non-deleted attributes wil be returned. | [optional][default to false]
**exclude_local_tags** | Option<**bool**> | Exclude local tags from the export | [optional]
**date** | Option<**String**> | You can use any of the valid time related filters (examples: 7d, timestamps, [14d, 7d] for ranges, etc.) | [optional]
**include_sightingdb** | Option<**bool**> | Extend response with Sightings DB results if the module is enabled | [optional]
**tag** | Option<**String**> |  | [optional]
**object_relation** | Option<**String**> | Filter by the attribute object relation value | [optional]
**threat_level_id** | Option<[**models::ThreatLevelId**](ThreatLevelId.md)> |  | [optional]
**return_format** | Option<[**models::EventsRestSearchReturnFormat**](EventsRestSearchReturnFormat.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


