# EventRestSearchListInnerEvent

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | **String** |  | 
**org_id** | Option<**String**> |  | [optional]
**distribution** | Option<[**models::DistributionLevelId**](DistributionLevelId.md)> |  | [optional]
**info** | Option<**String**> |  | [optional]
**orgc_id** | Option<**String**> |  | [optional]
**uuid** | [**uuid::Uuid**](uuid::Uuid.md) |  | 
**date** | Option<**String**> |  | [optional]
**published** | **bool** |  | [default to false]
**analysis** | Option<[**models::AnalysisLevelId**](AnalysisLevelId.md)> |  | [optional]
**attribute_count** | Option<**String**> |  | [optional]
**timestamp** | **String** |  | [default to 0]
**sharing_group_id** | Option<**String**> |  | [optional]
**proposal_email_lock** | Option<**bool**> |  | [optional]
**locked** | Option<**bool**> |  | [optional]
**threat_level_id** | Option<[**models::ThreatLevelId**](ThreatLevelId.md)> |  | [optional]
**publish_timestamp** | Option<**String**> |  | [optional][default to 0]
**sighting_timestamp** | **String** |  | [default to 0]
**disable_correlation** | Option<**bool**> |  | [optional][default to false]
**extends_uuid** | Option<**String**> |  | [optional]
**event_creator_email** | Option<**String**> |  | [optional]
**feed** | Option<[**models::Feed**](Feed.md)> |  | [optional]
**org** | Option<[**models::EventOrganisation**](EventOrganisation.md)> |  | [optional]
**orgc** | Option<[**models::EventOrganisation**](EventOrganisation.md)> |  | [optional]
**attribute** | Option<[**Vec<models::Attribute>**](Attribute.md)> |  | [optional]
**shadow_attribute** | Option<[**Vec<models::Attribute>**](Attribute.md)> |  | [optional]
**related_event** | Option<[**Vec<models::GetEventById200Response>**](getEventById_200_response.md)> |  | [optional]
**galaxy** | Option<[**Vec<models::Galaxy>**](Galaxy.md)> |  | [optional]
**object** | Option<[**Vec<models::Object>**](Object.md)> |  | [optional]
**event_report** | Option<[**Vec<models::EventReport>**](EventReport.md)> |  | [optional]
**tag** | Option<[**Vec<models::Tag>**](Tag.md)> |  | [optional]
**orgc_uuid** | Option<[**uuid::Uuid**](uuid::Uuid.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


