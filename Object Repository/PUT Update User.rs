<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>PUT Update User</name>
   <tag></tag>
   <elementGuidId>fbb5898e-3880-4f52-9010-5ba5cb86ea35</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <autoUpdateContent>false</autoUpdateContent>
   <connectionTimeout>-1</connectionTimeout>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;{\n    \&quot;name\&quot;: \&quot;${username}\&quot;,\n    \&quot;job\&quot;: \&quot;${password}\&quot;\n}&quot;,
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
      <webElementGuid>6f6ed1a0-3e3e-4b14-90ae-53ce2db6880b</webElementGuid>
   </httpHeaderProperties>
   <katalonVersion>8.6.8</katalonVersion>
   <maxResponseSize>-1</maxResponseSize>
   <migratedVersion>5.4.1</migratedVersion>
   <restRequestMethod>PUT</restRequestMethod>
   <restUrl>${Base_URL}/api/users/${id}</restUrl>
   <serviceType>RESTful</serviceType>
   <soapBody></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod></soapRequestMethod>
   <soapServiceEndpoint></soapServiceEndpoint>
   <soapServiceFunction></soapServiceFunction>
   <socketTimeout>-1</socketTimeout>
   <useServiceInfoFromWsdl>true</useServiceInfoFromWsdl>
   <variables>
      <defaultValue>GlobalVariable.Base_URL</defaultValue>
      <description></description>
      <id>a443d6f2-2b4d-4145-b0e1-e17c5d958f07</id>
      <masked>false</masked>
      <name>Base_URL</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.id</defaultValue>
      <description></description>
      <id>3a7b8fab-83da-4362-952b-5044cb89614d</id>
      <masked>false</masked>
      <name>id</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.name</defaultValue>
      <description></description>
      <id>60c751b4-b98c-4b4a-995b-b948fd4f2c99</id>
      <masked>false</masked>
      <name>name</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.job</defaultValue>
      <description></description>
      <id>e2bb1dab-ed1a-4e31-9a18-7438bfbf353e</id>
      <masked>false</masked>
      <name>job</name>
   </variables>
   <verificationScript>import static org.assertj.core.api.Assertions.*

import com.kms.katalon.core.testobject.RequestObject
import com.kms.katalon.core.testobject.ResponseObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webservice.verification.WSResponseManager

import groovy.json.JsonSlurper
import internal.GlobalVariable as GlobalVariable

RequestObject request = WSResponseManager.getInstance().getCurrentRequest()

ResponseObject response = WSResponseManager.getInstance().getCurrentResponse()</verificationScript>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
