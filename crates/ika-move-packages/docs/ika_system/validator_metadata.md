---
title: Module `(ika_system=0x0)::validator_metadata`
---

Metadata that describes a validator. Attached to the <code>StakingPool</code>


-  [Struct `ValidatorMetadata`](#(ika_system=0x0)_validator_metadata_ValidatorMetadata)
-  [Function `new`](#(ika_system=0x0)_validator_metadata_new)
-  [Function `set_image_url`](#(ika_system=0x0)_validator_metadata_set_image_url)
-  [Function `set_project_url`](#(ika_system=0x0)_validator_metadata_set_project_url)
-  [Function `set_description`](#(ika_system=0x0)_validator_metadata_set_description)
-  [Function `set_extra_fields`](#(ika_system=0x0)_validator_metadata_set_extra_fields)
-  [Function `image_url`](#(ika_system=0x0)_validator_metadata_image_url)
-  [Function `project_url`](#(ika_system=0x0)_validator_metadata_project_url)
-  [Function `description`](#(ika_system=0x0)_validator_metadata_description)
-  [Function `extra_fields`](#(ika_system=0x0)_validator_metadata_extra_fields)
-  [Function `default`](#(ika_system=0x0)_validator_metadata_default)


<pre><code><b>use</b> <a href="../std/ascii.md#std_ascii">std::ascii</a>;
<b>use</b> <a href="../std/option.md#std_option">std::option</a>;
<b>use</b> <a href="../std/string.md#std_string">std::string</a>;
<b>use</b> <a href="../std/vector.md#std_vector">std::vector</a>;
<b>use</b> <a href="../sui/vec_map.md#sui_vec_map">sui::vec_map</a>;
</code></pre>



<a name="(ika_system=0x0)_validator_metadata_ValidatorMetadata"></a>

## Struct `ValidatorMetadata`

Standard metadata for a validator. Created during the validator registration.


<pre><code><b>public</b> <b>struct</b> <a href="../ika_system/validator_metadata.md#(ika_system=0x0)_validator_metadata_ValidatorMetadata">ValidatorMetadata</a> <b>has</b> <b>copy</b>, drop, store
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code><a href="../ika_system/validator_metadata.md#(ika_system=0x0)_validator_metadata_image_url">image_url</a>: <a href="../std/string.md#std_string_String">std::string::String</a></code>
</dt>
<dd>
</dd>
<dt>
<code><a href="../ika_system/validator_metadata.md#(ika_system=0x0)_validator_metadata_project_url">project_url</a>: <a href="../std/string.md#std_string_String">std::string::String</a></code>
</dt>
<dd>
</dd>
<dt>
<code><a href="../ika_system/validator_metadata.md#(ika_system=0x0)_validator_metadata_description">description</a>: <a href="../std/string.md#std_string_String">std::string::String</a></code>
</dt>
<dd>
</dd>
<dt>
<code><a href="../ika_system/validator_metadata.md#(ika_system=0x0)_validator_metadata_extra_fields">extra_fields</a>: <a href="../sui/vec_map.md#sui_vec_map_VecMap">sui::vec_map::VecMap</a>&lt;<a href="../std/string.md#std_string_String">std::string::String</a>, <a href="../std/string.md#std_string_String">std::string::String</a>&gt;</code>
</dt>
<dd>
</dd>
</dl>


</details>

<a name="(ika_system=0x0)_validator_metadata_new"></a>

## Function `new`

Create a new <code><a href="../ika_system/validator_metadata.md#(ika_system=0x0)_validator_metadata_ValidatorMetadata">ValidatorMetadata</a></code> instance


<pre><code><b>public</b> <b>fun</b> <a href="../ika_system/validator_metadata.md#(ika_system=0x0)_validator_metadata_new">new</a>(<a href="../ika_system/validator_metadata.md#(ika_system=0x0)_validator_metadata_image_url">image_url</a>: <a href="../std/string.md#std_string_String">std::string::String</a>, <a href="../ika_system/validator_metadata.md#(ika_system=0x0)_validator_metadata_project_url">project_url</a>: <a href="../std/string.md#std_string_String">std::string::String</a>, <a href="../ika_system/validator_metadata.md#(ika_system=0x0)_validator_metadata_description">description</a>: <a href="../std/string.md#std_string_String">std::string::String</a>): (ika_system=0x0)::<a href="../ika_system/validator_metadata.md#(ika_system=0x0)_validator_metadata_ValidatorMetadata">validator_metadata::ValidatorMetadata</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../ika_system/validator_metadata.md#(ika_system=0x0)_validator_metadata_new">new</a>(<a href="../ika_system/validator_metadata.md#(ika_system=0x0)_validator_metadata_image_url">image_url</a>: String, <a href="../ika_system/validator_metadata.md#(ika_system=0x0)_validator_metadata_project_url">project_url</a>: String, <a href="../ika_system/validator_metadata.md#(ika_system=0x0)_validator_metadata_description">description</a>: String): <a href="../ika_system/validator_metadata.md#(ika_system=0x0)_validator_metadata_ValidatorMetadata">ValidatorMetadata</a> {
    <a href="../ika_system/validator_metadata.md#(ika_system=0x0)_validator_metadata_ValidatorMetadata">ValidatorMetadata</a> {
        <a href="../ika_system/validator_metadata.md#(ika_system=0x0)_validator_metadata_image_url">image_url</a>,
        <a href="../ika_system/validator_metadata.md#(ika_system=0x0)_validator_metadata_project_url">project_url</a>,
        <a href="../ika_system/validator_metadata.md#(ika_system=0x0)_validator_metadata_description">description</a>,
        <a href="../ika_system/validator_metadata.md#(ika_system=0x0)_validator_metadata_extra_fields">extra_fields</a>: vec_map::empty(),
    }
}
</code></pre>



</details>

<a name="(ika_system=0x0)_validator_metadata_set_image_url"></a>

## Function `set_image_url`

Set the image URL of the Validator.


<pre><code><b>public</b> <b>fun</b> <a href="../ika_system/validator_metadata.md#(ika_system=0x0)_validator_metadata_set_image_url">set_image_url</a>(metadata: &<b>mut</b> (ika_system=0x0)::<a href="../ika_system/validator_metadata.md#(ika_system=0x0)_validator_metadata_ValidatorMetadata">validator_metadata::ValidatorMetadata</a>, <a href="../ika_system/validator_metadata.md#(ika_system=0x0)_validator_metadata_image_url">image_url</a>: <a href="../std/string.md#std_string_String">std::string::String</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../ika_system/validator_metadata.md#(ika_system=0x0)_validator_metadata_set_image_url">set_image_url</a>(metadata: &<b>mut</b> <a href="../ika_system/validator_metadata.md#(ika_system=0x0)_validator_metadata_ValidatorMetadata">ValidatorMetadata</a>, <a href="../ika_system/validator_metadata.md#(ika_system=0x0)_validator_metadata_image_url">image_url</a>: String) {
    metadata.<a href="../ika_system/validator_metadata.md#(ika_system=0x0)_validator_metadata_image_url">image_url</a> = <a href="../ika_system/validator_metadata.md#(ika_system=0x0)_validator_metadata_image_url">image_url</a>;
}
</code></pre>



</details>

<a name="(ika_system=0x0)_validator_metadata_set_project_url"></a>

## Function `set_project_url`

Set the project URL of the Validator.


<pre><code><b>public</b> <b>fun</b> <a href="../ika_system/validator_metadata.md#(ika_system=0x0)_validator_metadata_set_project_url">set_project_url</a>(metadata: &<b>mut</b> (ika_system=0x0)::<a href="../ika_system/validator_metadata.md#(ika_system=0x0)_validator_metadata_ValidatorMetadata">validator_metadata::ValidatorMetadata</a>, <a href="../ika_system/validator_metadata.md#(ika_system=0x0)_validator_metadata_project_url">project_url</a>: <a href="../std/string.md#std_string_String">std::string::String</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../ika_system/validator_metadata.md#(ika_system=0x0)_validator_metadata_set_project_url">set_project_url</a>(metadata: &<b>mut</b> <a href="../ika_system/validator_metadata.md#(ika_system=0x0)_validator_metadata_ValidatorMetadata">ValidatorMetadata</a>, <a href="../ika_system/validator_metadata.md#(ika_system=0x0)_validator_metadata_project_url">project_url</a>: String) {
    metadata.<a href="../ika_system/validator_metadata.md#(ika_system=0x0)_validator_metadata_project_url">project_url</a> = <a href="../ika_system/validator_metadata.md#(ika_system=0x0)_validator_metadata_project_url">project_url</a>;
}
</code></pre>



</details>

<a name="(ika_system=0x0)_validator_metadata_set_description"></a>

## Function `set_description`

Set the description of the Validator.


<pre><code><b>public</b> <b>fun</b> <a href="../ika_system/validator_metadata.md#(ika_system=0x0)_validator_metadata_set_description">set_description</a>(metadata: &<b>mut</b> (ika_system=0x0)::<a href="../ika_system/validator_metadata.md#(ika_system=0x0)_validator_metadata_ValidatorMetadata">validator_metadata::ValidatorMetadata</a>, <a href="../ika_system/validator_metadata.md#(ika_system=0x0)_validator_metadata_description">description</a>: <a href="../std/string.md#std_string_String">std::string::String</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../ika_system/validator_metadata.md#(ika_system=0x0)_validator_metadata_set_description">set_description</a>(metadata: &<b>mut</b> <a href="../ika_system/validator_metadata.md#(ika_system=0x0)_validator_metadata_ValidatorMetadata">ValidatorMetadata</a>, <a href="../ika_system/validator_metadata.md#(ika_system=0x0)_validator_metadata_description">description</a>: String) {
    metadata.<a href="../ika_system/validator_metadata.md#(ika_system=0x0)_validator_metadata_description">description</a> = <a href="../ika_system/validator_metadata.md#(ika_system=0x0)_validator_metadata_description">description</a>;
}
</code></pre>



</details>

<a name="(ika_system=0x0)_validator_metadata_set_extra_fields"></a>

## Function `set_extra_fields`

Set an extra field of the Validator.


<pre><code><b>public</b> <b>fun</b> <a href="../ika_system/validator_metadata.md#(ika_system=0x0)_validator_metadata_set_extra_fields">set_extra_fields</a>(metadata: &<b>mut</b> (ika_system=0x0)::<a href="../ika_system/validator_metadata.md#(ika_system=0x0)_validator_metadata_ValidatorMetadata">validator_metadata::ValidatorMetadata</a>, <a href="../ika_system/validator_metadata.md#(ika_system=0x0)_validator_metadata_extra_fields">extra_fields</a>: <a href="../sui/vec_map.md#sui_vec_map_VecMap">sui::vec_map::VecMap</a>&lt;<a href="../std/string.md#std_string_String">std::string::String</a>, <a href="../std/string.md#std_string_String">std::string::String</a>&gt;)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../ika_system/validator_metadata.md#(ika_system=0x0)_validator_metadata_set_extra_fields">set_extra_fields</a>(metadata: &<b>mut</b> <a href="../ika_system/validator_metadata.md#(ika_system=0x0)_validator_metadata_ValidatorMetadata">ValidatorMetadata</a>, <a href="../ika_system/validator_metadata.md#(ika_system=0x0)_validator_metadata_extra_fields">extra_fields</a>: VecMap&lt;String, String&gt;) {
    metadata.<a href="../ika_system/validator_metadata.md#(ika_system=0x0)_validator_metadata_extra_fields">extra_fields</a> = <a href="../ika_system/validator_metadata.md#(ika_system=0x0)_validator_metadata_extra_fields">extra_fields</a>;
}
</code></pre>



</details>

<a name="(ika_system=0x0)_validator_metadata_image_url"></a>

## Function `image_url`

Returns the image URL of the Validator.


<pre><code><b>public</b> <b>fun</b> <a href="../ika_system/validator_metadata.md#(ika_system=0x0)_validator_metadata_image_url">image_url</a>(metadata: &(ika_system=0x0)::<a href="../ika_system/validator_metadata.md#(ika_system=0x0)_validator_metadata_ValidatorMetadata">validator_metadata::ValidatorMetadata</a>): <a href="../std/string.md#std_string_String">std::string::String</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../ika_system/validator_metadata.md#(ika_system=0x0)_validator_metadata_image_url">image_url</a>(metadata: &<a href="../ika_system/validator_metadata.md#(ika_system=0x0)_validator_metadata_ValidatorMetadata">ValidatorMetadata</a>): String { metadata.<a href="../ika_system/validator_metadata.md#(ika_system=0x0)_validator_metadata_image_url">image_url</a> }
</code></pre>



</details>

<a name="(ika_system=0x0)_validator_metadata_project_url"></a>

## Function `project_url`

Returns the project URL of the Validator.


<pre><code><b>public</b> <b>fun</b> <a href="../ika_system/validator_metadata.md#(ika_system=0x0)_validator_metadata_project_url">project_url</a>(metadata: &(ika_system=0x0)::<a href="../ika_system/validator_metadata.md#(ika_system=0x0)_validator_metadata_ValidatorMetadata">validator_metadata::ValidatorMetadata</a>): <a href="../std/string.md#std_string_String">std::string::String</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../ika_system/validator_metadata.md#(ika_system=0x0)_validator_metadata_project_url">project_url</a>(metadata: &<a href="../ika_system/validator_metadata.md#(ika_system=0x0)_validator_metadata_ValidatorMetadata">ValidatorMetadata</a>): String { metadata.<a href="../ika_system/validator_metadata.md#(ika_system=0x0)_validator_metadata_project_url">project_url</a> }
</code></pre>



</details>

<a name="(ika_system=0x0)_validator_metadata_description"></a>

## Function `description`

Returns the description of the Validator.


<pre><code><b>public</b> <b>fun</b> <a href="../ika_system/validator_metadata.md#(ika_system=0x0)_validator_metadata_description">description</a>(metadata: &(ika_system=0x0)::<a href="../ika_system/validator_metadata.md#(ika_system=0x0)_validator_metadata_ValidatorMetadata">validator_metadata::ValidatorMetadata</a>): <a href="../std/string.md#std_string_String">std::string::String</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../ika_system/validator_metadata.md#(ika_system=0x0)_validator_metadata_description">description</a>(metadata: &<a href="../ika_system/validator_metadata.md#(ika_system=0x0)_validator_metadata_ValidatorMetadata">ValidatorMetadata</a>): String { metadata.<a href="../ika_system/validator_metadata.md#(ika_system=0x0)_validator_metadata_description">description</a> }
</code></pre>



</details>

<a name="(ika_system=0x0)_validator_metadata_extra_fields"></a>

## Function `extra_fields`

Returns the extra fields of the Validator.


<pre><code><b>public</b> <b>fun</b> <a href="../ika_system/validator_metadata.md#(ika_system=0x0)_validator_metadata_extra_fields">extra_fields</a>(metadata: &(ika_system=0x0)::<a href="../ika_system/validator_metadata.md#(ika_system=0x0)_validator_metadata_ValidatorMetadata">validator_metadata::ValidatorMetadata</a>): &<a href="../sui/vec_map.md#sui_vec_map_VecMap">sui::vec_map::VecMap</a>&lt;<a href="../std/string.md#std_string_String">std::string::String</a>, <a href="../std/string.md#std_string_String">std::string::String</a>&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../ika_system/validator_metadata.md#(ika_system=0x0)_validator_metadata_extra_fields">extra_fields</a>(metadata: &<a href="../ika_system/validator_metadata.md#(ika_system=0x0)_validator_metadata_ValidatorMetadata">ValidatorMetadata</a>): &VecMap&lt;String, String&gt; {
    &metadata.<a href="../ika_system/validator_metadata.md#(ika_system=0x0)_validator_metadata_extra_fields">extra_fields</a>
}
</code></pre>



</details>

<a name="(ika_system=0x0)_validator_metadata_default"></a>

## Function `default`

Create a default empty <code><a href="../ika_system/validator_metadata.md#(ika_system=0x0)_validator_metadata_ValidatorMetadata">ValidatorMetadata</a></code> instance.


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/validator_metadata.md#(ika_system=0x0)_validator_metadata_default">default</a>(): (ika_system=0x0)::<a href="../ika_system/validator_metadata.md#(ika_system=0x0)_validator_metadata_ValidatorMetadata">validator_metadata::ValidatorMetadata</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/validator_metadata.md#(ika_system=0x0)_validator_metadata_default">default</a>(): <a href="../ika_system/validator_metadata.md#(ika_system=0x0)_validator_metadata_ValidatorMetadata">ValidatorMetadata</a> {
    <a href="../ika_system/validator_metadata.md#(ika_system=0x0)_validator_metadata_ValidatorMetadata">ValidatorMetadata</a> {
        <a href="../ika_system/validator_metadata.md#(ika_system=0x0)_validator_metadata_image_url">image_url</a>: b"".to_string(),
        <a href="../ika_system/validator_metadata.md#(ika_system=0x0)_validator_metadata_project_url">project_url</a>: b"".to_string(),
        <a href="../ika_system/validator_metadata.md#(ika_system=0x0)_validator_metadata_description">description</a>: b"".to_string(),
        <a href="../ika_system/validator_metadata.md#(ika_system=0x0)_validator_metadata_extra_fields">extra_fields</a>: vec_map::empty(),
    }
}
</code></pre>



</details>
