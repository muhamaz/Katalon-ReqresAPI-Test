<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>GET List Users</name>
   <tag></tag>
   <elementGuidId>c8ef5f81-f0fb-45bc-aed6-be13d58cc3ec</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <autoUpdateContent>true</autoUpdateContent>
   <connectionTimeout>0</connectionTimeout>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent></httpBodyContent>
   <httpBodyType></httpBodyType>
   <katalonVersion>8.6.8</katalonVersion>
   <maxResponseSize>0</maxResponseSize>
   <migratedVersion>5.4.1</migratedVersion>
   <restRequestMethod>GET</restRequestMethod>
   <restUrl>${Base_URL}/api/users?page=${Page}</restUrl>
   <serviceType>RESTful</serviceType>
   <soapBody></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod></soapRequestMethod>
   <soapServiceEndpoint></soapServiceEndpoint>
   <soapServiceFunction></soapServiceFunction>
   <socketTimeout>0</socketTimeout>
   <useServiceInfoFromWsdl>true</useServiceInfoFromWsdl>
   <variables>
      <defaultValue>GlobalVariable.Base_URL</defaultValue>
      <description></description>
      <id>5fa47ab2-1720-47ee-8103-e325e15bc14a</id>
      <masked>false</masked>
      <name>Base_URL</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.Page</defaultValue>
      <description></description>
      <id>94ea4717-abf6-4a59-8140-e6903f51a21f</id>
      <masked>false</masked>
      <name>Page</name>
   </variables>
   <verificationScript>import static org.assertj.core.api.Assertions.*

import com.kms.katalon.core.testobject.RequestObject
import com.kms.katalon.core.testobject.ResponseObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webservice.verification.WSResponseManager

import groovy.json.JsonSlurper
import internal.GlobalVariable as GlobalVariable

RequestObject request = WSResponseManager.getInstance().getCurrentRequest()

ResponseObjectresponse = WSResponseManager.getInstance().getCurrentResponse()


</verificationScript>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
