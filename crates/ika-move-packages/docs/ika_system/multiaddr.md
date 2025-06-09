---
title: Module `(ika_system=0x0)::multiaddr`
---

This module provides utilities for validating multiaddr strings in Sui Move.
Multiaddr is a format for encoding addresses from various well-established network protocols.
This implementation supports validation for:
- IPv4 addresses with TCP/UDP
- IPv6 addresses with TCP/UDP
- DNS hostnames with TCP/UDP
- HTTP protocol


-  [Function `validate_tcp`](#(ika_system=0x0)_multiaddr_validate_tcp)
    -  [Arguments](#@Arguments_0)
    -  [Returns](#@Returns_1)
    -  [Examples](#@Examples_2)
-  [Function `validate_udp`](#(ika_system=0x0)_multiaddr_validate_udp)
    -  [Arguments](#@Arguments_3)
    -  [Returns](#@Returns_4)
    -  [Examples](#@Examples_5)
-  [Function `validate_with_transport`](#(ika_system=0x0)_multiaddr_validate_with_transport)
    -  [Arguments](#@Arguments_6)
    -  [Returns](#@Returns_7)
-  [Function `is_valid_ipv4`](#(ika_system=0x0)_multiaddr_is_valid_ipv4)
    -  [Arguments](#@Arguments_8)
    -  [Returns](#@Returns_9)
-  [Function `is_valid_ipv6`](#(ika_system=0x0)_multiaddr_is_valid_ipv6)
    -  [Arguments](#@Arguments_10)
    -  [Returns](#@Returns_11)
-  [Function `is_valid_dns`](#(ika_system=0x0)_multiaddr_is_valid_dns)
    -  [Arguments](#@Arguments_12)
    -  [Returns](#@Returns_13)


<pre><code><b>use</b> <a href="../std/ascii.md#std_ascii">std::ascii</a>;
<b>use</b> <a href="../std/option.md#std_option">std::option</a>;
<b>use</b> <a href="../std/string.md#std_string">std::string</a>;
<b>use</b> <a href="../std/vector.md#std_vector">std::vector</a>;
</code></pre>



<a name="(ika_system=0x0)_multiaddr_validate_tcp"></a>

## Function `validate_tcp`

Validates a multiaddr string for TCP with any of IPv4/IPv6/DNS.


<a name="@Arguments_0"></a>

### Arguments

* <code>addr</code> - The multiaddr string to validate


<a name="@Returns_1"></a>

### Returns

* <code><b>true</b></code> if the multiaddr is valid for TCP, <code><b>false</b></code> otherwise


<a name="@Examples_2"></a>

### Examples

```
let valid_addr = string::utf8(b"/ip4/192.168.1.1/tcp/8080");
assert!(validate_tcp(&valid_addr));
```


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/multiaddr.md#(ika_system=0x0)_multiaddr_validate_tcp">validate_tcp</a>(addr: &<a href="../std/string.md#std_string_String">std::string::String</a>): bool
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/multiaddr.md#(ika_system=0x0)_multiaddr_validate_tcp">validate_tcp</a>(addr: &String): bool {
    <a href="../ika_system/multiaddr.md#(ika_system=0x0)_multiaddr_validate_with_transport">validate_with_transport</a>(addr, string::utf8(b"tcp"))
}
</code></pre>



</details>

<a name="(ika_system=0x0)_multiaddr_validate_udp"></a>

## Function `validate_udp`

Validates a multiaddr string for UDP with any of IPv4/IPv6/DNS.


<a name="@Arguments_3"></a>

### Arguments

* <code>addr</code> - The multiaddr string to validate


<a name="@Returns_4"></a>

### Returns

* <code><b>true</b></code> if the multiaddr is valid for UDP, <code><b>false</b></code> otherwise


<a name="@Examples_5"></a>

### Examples

```
let valid_addr = string::utf8(b"/ip4/192.168.1.1/udp/8080");
assert!(validate_udp(&valid_addr));
```


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/multiaddr.md#(ika_system=0x0)_multiaddr_validate_udp">validate_udp</a>(addr: &<a href="../std/string.md#std_string_String">std::string::String</a>): bool
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../ika_system/multiaddr.md#(ika_system=0x0)_multiaddr_validate_udp">validate_udp</a>(addr: &String): bool {
    <a href="../ika_system/multiaddr.md#(ika_system=0x0)_multiaddr_validate_with_transport">validate_with_transport</a>(addr, string::utf8(b"udp"))
}
</code></pre>



</details>

<a name="(ika_system=0x0)_multiaddr_validate_with_transport"></a>

## Function `validate_with_transport`

Internal helper function to validate multiaddr with a specific transport protocol.


<a name="@Arguments_6"></a>

### Arguments

* <code>addr</code> - The multiaddr string to validate
* <code>transport</code> - The expected transport protocol (tcp, udp, or quic)


<a name="@Returns_7"></a>

### Returns

* <code><b>true</b></code> if the multiaddr is valid for the given transport, <code><b>false</b></code> otherwise


<pre><code><b>fun</b> <a href="../ika_system/multiaddr.md#(ika_system=0x0)_multiaddr_validate_with_transport">validate_with_transport</a>(addr: &<a href="../std/string.md#std_string_String">std::string::String</a>, transport: <a href="../std/string.md#std_string_String">std::string::String</a>): bool
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../ika_system/multiaddr.md#(ika_system=0x0)_multiaddr_validate_with_transport">validate_with_transport</a>(addr: &String, transport: String): bool {
    <b>let</b> bytes = string::as_bytes(addr);
    <b>let</b> len = vector::length(bytes);
    <b>if</b> (len &lt; 1) <b>return</b> <b>false</b>;
    // Check <b>if</b> it starts with a slash
    <b>if</b> (*vector::borrow(bytes, 0) != 47) <b>return</b> <b>false</b>; // ASCII '/' is 47
    // Find the parts by iterating through the string once
    <b>let</b> <b>mut</b> part_start = 1; // Skip first slash
    <b>let</b> <b>mut</b> part_num = 0;
    <b>let</b> <b>mut</b> protocol = string::utf8(b"");
    <b>let</b> <b>mut</b> <b>address</b> = string::utf8(b"");
    <b>let</b> <b>mut</b> actual_transport = string::utf8(b"");
    <b>let</b> <b>mut</b> port = string::utf8(b"");
    <b>let</b> <b>mut</b> i = 1;
    <b>while</b> (i &lt; len) {
        <b>if</b> (*vector::borrow(bytes, i) == 47 || i == len - 1) {
            <b>let</b> end = <b>if</b> (i == len - 1) i + 1 <b>else</b> i;
            <b>let</b> part = string::substring(addr, part_start, end);
            <b>if</b> (part_num == 0) {
                protocol = part;
            } <b>else</b> <b>if</b> (part_num == 1) {
                <b>address</b> = part;
            } <b>else</b> <b>if</b> (part_num == 2) {
                actual_transport = part;
            } <b>else</b> <b>if</b> (part_num == 3) {
                port = part;
            } <b>else</b> {
                // For additional segments, we only validate <b>if</b> they are HTTP/HTTPS resources
                // or <b>if</b> they are valid protocol names (http, https, quic)
                <b>let</b> http = string::utf8(b"http");
                <b>let</b> https = string::utf8(b"https");
                <b>let</b> quic = string::utf8(b"quic");
                // If this is a protocol name, validate it
                <b>if</b> (part == http || part == https || part == quic) {
                    // Valid protocol name, <b>continue</b>
                } <b>else</b> {
                    // This is either a resource path or an unknown protocol
                    // If it's a resource path, it can contain any valid URL characters
                    // If it's an unknown protocol, we should reject it
                    <b>if</b> (part_num == 4) {
                        // First additional segment must be a known protocol
                        <b>return</b> <b>false</b>
                    };
                    // For subsequent segments, assume they are resource paths
                    <b>break</b>
                };
            };
            part_start = i + 1;
            part_num = part_num + 1;
        };
        i = i + 1;
    };
    <b>if</b> (part_num &lt; 4) <b>return</b> <b>false</b>; // Need at least protocol/<b>address</b>/transport/port
    // Validate protocol
    <b>let</b> ip4 = string::utf8(b"ip4");
    <b>let</b> ip6 = string::utf8(b"ip6");
    <b>let</b> dns4 = string::utf8(b"dns4");
    <b>let</b> dns6 = string::utf8(b"dns6");
    <b>let</b> dns = string::utf8(b"dns");
    <b>if</b> (protocol != ip4 &&
        protocol != ip6 &&
        protocol != dns4 &&
        protocol != dns6 &&
        protocol != dns) <b>return</b> <b>false</b>;
    // Validate <b>address</b> based on protocol
    <b>if</b> (protocol == ip4) {
        <b>if</b> (!<a href="../ika_system/multiaddr.md#(ika_system=0x0)_multiaddr_is_valid_ipv4">is_valid_ipv4</a>(&<b>address</b>)) <b>return</b> <b>false</b>;
    } <b>else</b> <b>if</b> (protocol == ip6) {
        <b>if</b> (!<a href="../ika_system/multiaddr.md#(ika_system=0x0)_multiaddr_is_valid_ipv6">is_valid_ipv6</a>(&<b>address</b>)) <b>return</b> <b>false</b>;
    } <b>else</b> <b>if</b> (protocol == dns4 || protocol == dns6 || protocol == dns) {
        <b>if</b> (!<a href="../ika_system/multiaddr.md#(ika_system=0x0)_multiaddr_is_valid_dns">is_valid_dns</a>(&<b>address</b>)) <b>return</b> <b>false</b>;
    };
    // Validate transport
    <b>if</b> (actual_transport != transport) <b>return</b> <b>false</b>;
    // Validate port - must be a string of digits
    <b>if</b> (string::length(&port) == 0) <b>return</b> <b>false</b>;
    <b>let</b> port_bytes = string::as_bytes(&port);
    <b>let</b> port_len = vector::length(port_bytes);
    <b>let</b> <b>mut</b> j = 0;
    <b>let</b> <b>mut</b> is_valid_port = <b>true</b>;
    <b>while</b> (j &lt; port_len) {
        <b>let</b> byte = *vector::borrow(port_bytes, j);
        <b>if</b> (byte &lt; 48 || byte &gt; 57) {
            is_valid_port = <b>false</b>;
            <b>break</b>
        };
        j = j + 1;
    };
    is_valid_port
}
</code></pre>



</details>

<a name="(ika_system=0x0)_multiaddr_is_valid_ipv4"></a>

## Function `is_valid_ipv4`

Validates an IPv4 address format.


<a name="@Arguments_8"></a>

### Arguments

* <code>ip</code> - The IPv4 address string to validate


<a name="@Returns_9"></a>

### Returns

* <code><b>true</b></code> if the IPv4 address is valid, <code><b>false</b></code> otherwise


<pre><code><b>fun</b> <a href="../ika_system/multiaddr.md#(ika_system=0x0)_multiaddr_is_valid_ipv4">is_valid_ipv4</a>(ip: &<a href="../std/string.md#std_string_String">std::string::String</a>): bool
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../ika_system/multiaddr.md#(ika_system=0x0)_multiaddr_is_valid_ipv4">is_valid_ipv4</a>(ip: &String): bool {
    <b>let</b> len = ip.length();
    <b>let</b> <b>mut</b> parts: vector&lt;String&gt; = vector::empty();
    <b>let</b> <b>mut</b> start = 0;
    <b>let</b> <b>mut</b> i = 0;
    <b>while</b> (i &lt; len) {
        <b>let</b> current = ip.substring(i, i + 1);
        <b>if</b> (current == string::utf8(b".")) {
            <b>let</b> part = ip.substring(start, i);
            parts.push_back(part);
            start = i + 1;
        };
        i = i + 1;
    };
    // Add last part
    <b>let</b> last_part = ip.substring(start, len);
    parts.push_back(last_part);
    <b>if</b> (parts.length() != 4) <b>return</b> <b>false</b>;
    <b>let</b> <b>mut</b> i = 0;
    <b>while</b> (i &lt; 4) {
        <b>let</b> octet = parts.borrow(i);
        <b>let</b> octet_bytes = octet.as_bytes();
        <b>let</b> octet_len = octet_bytes.length();
        // Check <b>if</b> octet is empty or too long
        <b>if</b> (octet_len == 0 || octet_len &gt; 3) <b>return</b> <b>false</b>;
        // Check <b>if</b> all characters are digits
        <b>let</b> <b>mut</b> j = 0;
        <b>while</b> (j &lt; octet_len) {
            <b>let</b> byte = *octet_bytes.borrow(j);
            <b>if</b> (byte &lt; 48 || byte &gt; 57) <b>return</b> <b>false</b>; // Not a digit
            j = j + 1;
        };
        // Check <b>if</b> number is too large
        <b>if</b> (octet_len == 3) {
            <b>let</b> first = *octet_bytes.borrow(0);
            <b>let</b> second = *octet_bytes.borrow(1);
            <b>let</b> third = *octet_bytes.borrow(2);
            <b>if</b> (first &gt; 50) <b>return</b> <b>false</b>; // First digit &gt; 2
            <b>if</b> (first == 50) { // First digit is 2
                <b>if</b> (second &gt; 53) <b>return</b> <b>false</b>; // Second digit &gt; 5
                <b>if</b> (second == 53 && third &gt; 53) <b>return</b> <b>false</b>; // Second digit is 5 and third &gt; 5
            };
        } <b>else</b> <b>if</b> (octet_len == 2) {
            <b>let</b> first = *octet_bytes.borrow(0);
            <b>let</b> second = *octet_bytes.borrow(1);
            <b>if</b> (first &gt; 50) <b>return</b> <b>false</b>; // First digit &gt; 2
            <b>if</b> (first == 50 && second &gt; 53) <b>return</b> <b>false</b>; // First digit is 2 and second &gt; 5
        };
        i = i + 1;
    };
    <b>true</b>
}
</code></pre>



</details>

<a name="(ika_system=0x0)_multiaddr_is_valid_ipv6"></a>

## Function `is_valid_ipv6`

Validates an IPv6 address format.


<a name="@Arguments_10"></a>

### Arguments

* <code>ip</code> - The IPv6 address string to validate


<a name="@Returns_11"></a>

### Returns

* <code><b>true</b></code> if the IPv6 address is valid, <code><b>false</b></code> otherwise


<pre><code><b>fun</b> <a href="../ika_system/multiaddr.md#(ika_system=0x0)_multiaddr_is_valid_ipv6">is_valid_ipv6</a>(ip: &<a href="../std/string.md#std_string_String">std::string::String</a>): bool
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../ika_system/multiaddr.md#(ika_system=0x0)_multiaddr_is_valid_ipv6">is_valid_ipv6</a>(ip: &String): bool {
    <b>let</b> len = ip.length();
    <b>let</b> <b>mut</b> parts: vector&lt;String&gt; = vector::empty();
    <b>let</b> <b>mut</b> start = 0;
    <b>let</b> <b>mut</b> i = 0;
    <b>let</b> <b>mut</b> consecutive_colons = <b>false</b>;
    <b>let</b> <b>mut</b> has_double_colon = <b>false</b>;
    <b>while</b> (i &lt; len) {
        <b>let</b> current = ip.substring(i, i + 1);
        <b>if</b> (current == string::utf8(b":")) {
            <b>if</b> (i &gt; 0 && ip.substring(i - 1, i) == string::utf8(b":")) {
                <b>if</b> (has_double_colon) <b>return</b> <b>false</b>; // Only one :: allowed
                has_double_colon = <b>true</b>;
                consecutive_colons = <b>true</b>;
            } <b>else</b> {
                <b>if</b> (!consecutive_colons) {
                    <b>let</b> part = ip.substring(start, i);
                    <b>if</b> (part.length() &gt; 0) {
                        parts.push_back(part);
                    };
                };
                consecutive_colons = <b>false</b>;
            };
            start = i + 1;
        };
        i = i + 1;
    };
    // Add last part <b>if</b> not empty
    <b>let</b> last_part = ip.substring(start, len);
    <b>if</b> (last_part.length() &gt; 0) {
        parts.push_back(last_part);
    };
    <b>let</b> num_parts = parts.length();
    <b>if</b> (!has_double_colon && num_parts != 8) <b>return</b> <b>false</b>;
    <b>if</b> (has_double_colon && num_parts &gt;= 8) <b>return</b> <b>false</b>;
    <b>let</b> <b>mut</b> i = 0;
    <b>while</b> (i &lt; num_parts) {
        <b>let</b> segment = parts.borrow(i);
        <b>let</b> segment_len = segment.length();
        <b>if</b> (segment_len == 0 || segment_len &gt; 4) <b>return</b> <b>false</b>;
        // Validate hex characters
        <b>let</b> segment_bytes = segment.as_bytes();
        <b>let</b> <b>mut</b> j = 0;
        <b>while</b> (j &lt; segment_len) {
            <b>let</b> byte = *segment_bytes.borrow(j);
            <b>let</b> is_digit = byte &gt;= 48 && byte &lt;= 57; // 0-9
            <b>let</b> is_hex_lower = byte &gt;= 97 && byte &lt;= 102; // a-f
            <b>let</b> is_hex_upper = byte &gt;= 65 && byte &lt;= 70; // A-F
            <b>if</b> (!is_digit && !is_hex_lower && !is_hex_upper) <b>return</b> <b>false</b>;
            j = j + 1;
        };
        i = i + 1;
    };
    <b>true</b>
}
</code></pre>



</details>

<a name="(ika_system=0x0)_multiaddr_is_valid_dns"></a>

## Function `is_valid_dns`

Validates a DNS hostname format.


<a name="@Arguments_12"></a>

### Arguments

* <code>hostname</code> - The DNS hostname string to validate


<a name="@Returns_13"></a>

### Returns

* <code><b>true</b></code> if the DNS hostname is valid, <code><b>false</b></code> otherwise


<pre><code><b>fun</b> <a href="../ika_system/multiaddr.md#(ika_system=0x0)_multiaddr_is_valid_dns">is_valid_dns</a>(hostname: &<a href="../std/string.md#std_string_String">std::string::String</a>): bool
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../ika_system/multiaddr.md#(ika_system=0x0)_multiaddr_is_valid_dns">is_valid_dns</a>(hostname: &String): bool {
    <b>let</b> len = hostname.length();
    <b>if</b> (len &lt; 1 || len &gt; 253) <b>return</b> <b>false</b>;
    <b>let</b> <b>mut</b> parts: vector&lt;String&gt; = vector::empty();
    <b>let</b> <b>mut</b> start = 0;
    <b>let</b> <b>mut</b> i = 0;
    <b>while</b> (i &lt; len) {
        <b>let</b> current = hostname.substring(i, i + 1);
        <b>if</b> (current == string::utf8(b".")) {
            <b>let</b> part = hostname.substring(start, i);
            <b>if</b> (part.length() == 0) <b>return</b> <b>false</b>; // Empty label not allowed
            parts.push_back(part);
            start = i + 1;
        };
        i = i + 1;
    };
    // Add last part
    <b>let</b> last_part = hostname.substring(start, len);
    <b>if</b> (last_part.length() == 0) <b>return</b> <b>false</b>; // Empty label not allowed
    parts.push_back(last_part);
    <b>let</b> num_parts = parts.length();
    <b>if</b> (num_parts &lt; 1) <b>return</b> <b>false</b>;
    <b>let</b> <b>mut</b> i = 0;
    <b>while</b> (i &lt; num_parts) {
        <b>let</b> label = parts.borrow(i);
        <b>let</b> label_len = label.length();
        <b>if</b> (label_len &lt; 1 || label_len &gt; 63) <b>return</b> <b>false</b>;
        // Validate label characters
        <b>let</b> label_bytes = label.as_bytes();
        <b>let</b> <b>mut</b> j = 0;
        <b>while</b> (j &lt; label_len) {
            <b>let</b> byte = *label_bytes.borrow(j);
            <b>let</b> is_letter = (byte &gt;= 65 && byte &lt;= 90) || (byte &gt;= 97 && byte &lt;= 122); // A-Z or a-z
            <b>let</b> is_digit = byte &gt;= 48 && byte &lt;= 57; // 0-9
            <b>let</b> is_hyphen = byte == 45; // -
            <b>if</b> (!is_letter && !is_digit && !is_hyphen) <b>return</b> <b>false</b>;
            // First and last characters must be alphanumeric
            <b>if</b> (is_hyphen) {
                <b>if</b> (j == 0 || j == label_len - 1) <b>return</b> <b>false</b>;
            };
            j = j + 1;
        };
        i = i + 1;
    };
    <b>true</b>
}
</code></pre>



</details>
