<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>Test1</name>
   <tag></tag>
   <elementGuidId>a8499050-5734-4461-a0d3-c348e63f76af</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <connectionTimeout>-1</connectionTimeout>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;{\n  \&quot;age\&quot;:${age},\n  \&quot;avatar\&quot;:null,\n  \&quot;gender\&quot;:\&quot;${gender}\&quot;,\n  \&quot;password\&quot;:\&quot;${password}\&quot;,\n  \&quot;username\&quot;:\&quot;${username}\&quot;\n}&quot;,
  &quot;contentType&quot;: &quot;application/json&quot;,
  &quot;charset&quot;: &quot;UTF-8&quot;
}</httpBodyContent>
   <httpBodyType>text</httpBodyType>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Content-Type</name>
      <type>Main</type>
      <value>application/json</value>
      <webElementGuid>032c81e2-7fcb-4a9f-b5e0-79961d630c87</webElementGuid>
   </httpHeaderProperties>
   <katalonVersion>8.4.1</katalonVersion>
   <maxResponseSize>-1</maxResponseSize>
   <migratedVersion>5.4.1</migratedVersion>
   <restRequestMethod>POST</restRequestMethod>
   <restUrl>https://sample-web-service-aut.herokuapp.com/api/users/json</restUrl>
   <serviceType>RESTful</serviceType>
   <soapBody></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod></soapRequestMethod>
   <soapServiceEndpoint></soapServiceEndpoint>
   <soapServiceFunction></soapServiceFunction>
   <socketTimeout>-1</socketTimeout>
   <useServiceInfoFromWsdl>true</useServiceInfoFromWsdl>
   <variables>
      <defaultValue>'John Smith\r\n'</defaultValue>
      <description></description>
      <id>0b359e78-1adc-4c30-96a9-3af7e06edb66</id>
      <masked>false</masked>
      <name>username</name>
   </variables>
   <variables>
      <defaultValue>'123'</defaultValue>
      <description></description>
      <id>257bcb85-86ef-4226-8448-524de790ea68</id>
      <masked>false</masked>
      <name>password</name>
   </variables>
   <variables>
      <defaultValue>'MALE'</defaultValue>
      <description></description>
      <id>20ef4191-892a-48f8-959e-c69e88394288</id>
      <masked>false</masked>
      <name>gender</name>
   </variables>
   <variables>
      <defaultValue>21</defaultValue>
      <description></description>
      <id>0757272b-1c83-401f-9934-547d4d61a9f3</id>
      <masked>false</masked>
      <name>age</name>
   </variables>
   <variables>
      <defaultValue>null</defaultValue>
      <description></description>
      <id>84f937fd-8e8c-41eb-9610-12fed18c0073</id>
      <masked>false</masked>
      <name>avatar</name>
   </variables>
   <verificationScript>import static org.assertj.core.api.Assertions.*

import com.kms.katalon.core.testobject.RequestObject
import com.kms.katalon.core.testobject.ResponseObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webservice.verification.WSResponseManager

import groovy.json.JsonSlurper
import internal.GlobalVariable as GlobalVariable

RequestObject request = WSResponseManager.getInstance().getCurrentRequest()

ResponseObject response = WSResponseManager.getInstance().getCurrentResponse()


</verificationScript>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
