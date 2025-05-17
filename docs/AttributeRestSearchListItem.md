# AttributeRestSearchListItem

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<**String**> |  | [optional]
**event_id** | Option<**String**> |  | [optional]
**object_id** | Option<**String**> |  | [optional]
**object_relation** | Option<**String**> |  | [optional]
**category** | Option<[**models::AttributeCategory**](AttributeCategory.md)> |  | [optional]
**r#type** | Option<[**models::AttributeType**](AttributeType.md)> |  | [optional]
**value** | Option<**String**> |  | [optional]
**to_ids** | Option<**bool**> |  | [optional][default to true]
**uuid** | Option<[**uuid::Uuid**](uuid::Uuid.md)> |  | [optional]
**timestamp** | Option<**String**> |  | [optional][default to 0]
**distribution** | Option<[**models::DistributionLevelId**](DistributionLevelId.md)> |  | [optional]
**sharing_group_id** | Option<**String**> |  | [optional]
**comment** | Option<**String**> |  | [optional]
**deleted** | Option<**bool**> |  | [optional][default to false]
**disable_correlation** | Option<**bool**> |  | [optional][default to false]
**first_seen** | Option<[**models::AttributeNoIdFirstSeen**](AttributeNoId_first_seen.md)> |  | [optional]
**last_seen** | Option<[**models::AttributeNoIdFirstSeen**](AttributeNoId_first_seen.md)> |  | [optional]
**tag** | Option<[**Vec<models::Tag>**](Tag.md)> |  | [optional]
**galaxy** | Option<[**Vec<models::Galaxy>**](Galaxy.md)> |  | [optional]
**data** | Option<**String**> | base64 representation of the attachment | [optional]
**event_uuid** | Option<[**uuid::Uuid**](uuid::Uuid.md)> |  | [optional]
**decay_score** | Option<[**Vec<models::DecayScore>**](DecayScore.md)> |  | [optional]
**event** | Option<[**models::Event**](Event.md)> |  | [optional]
**object** | Option<[**models::Object**](Object.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


