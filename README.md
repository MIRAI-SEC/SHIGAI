# SHIGAI - Ransomware-Focused EDR

SHIGAI is an open-source GitHub project that aims to provide an simple Endpoint Detection and Response solution specifically designed to combat ransomware attacks. It leverages a combination of windows proprietary driver called Sysmon which monitor file precesses activities, enabling real-time analysis and detection of ransomware behaviors.

## Key Features

** Sysmon (System Monitor) is a Windows system service and device driver that provides advanced system activity logging. Developed by Microsoft, Sysmon is part of the Windows Sysinternals suite of utilities. It is designed to help system administrators, security professionals, and incident responders track and monitor detailed system activity to enhance security and aid in the detection of malicious behavior.

Here are some of the key functionalities and features of Sysmon:

Event Logging: Sysmon captures and logs various events and activities occurring on a Windows system. These events are recorded in the Windows Event Log, specifically under the "Microsoft-Windows-Sysmon" event source.

Detailed Information: Sysmon provides detailed information about processes, network connections, file creation, registry modifications, and more. This level of detail can be valuable for identifying suspicious or malicious activities.

Rule Configuration: Sysmon allows users to configure custom rules to specify what events to log. This enables fine-grained control over which types of activities to monitor and record.

Digital Signatures: Sysmon can verify the digital signatures of executable files to ensure they haven't been tampered with. It can log events related to unsigned or improperly signed executables.

Network Connection Monitoring: It logs information about network connections, including source and destination IP addresses and port numbers. This can help in identifying network-based attacks and unusual communication patterns.

Process Creation: Sysmon logs detailed information about process creation, including the parent process, command-line arguments, and the image (executable) file path. This is valuable for tracking the execution of potentially malicious processes.

Registry Activity: Registry modifications, such as changes to keys and values, are recorded by Sysmon. This can be useful for detecting unauthorized changes to the Windows Registry.

File Creation and Modification: Sysmon logs events related to file creation, deletion, and modification, including the file path and hash values. This can assist in identifying suspicious or unauthorized file changes.

Event Filtering: Sysmon supports event filtering, allowing users to exclude specific events or event sources from being logged, which can help reduce noise in the logs.

Integration with SIEM: Sysmon logs can be collected and forwarded to a Security Information and Event Management (SIEM) system, where they can be correlated with other security data for analysis and alerting.

- **Behavioral Anomaly Detection**: By monitoring the Sysmon driver, SHIGAI is capable of analyzing file system activities and detecting deviations from normal behavior, helping to identify potential ransomware activities.
- **Real-Time Alerting**: When suspicious or malicious behavior is detected in Sysmon , SHIGAI promptly generates alerts, allowing security teams to respond swiftly and mitigate potential damage.
- **Scalability**: SHIGAI is designed to handle large-scale deployments, making it suitable for enterprise-level environments with a high volume of endpoints.
- **Extensibility**: The project provides a flexible architecture that enables the addition of custom detection rules and integration with existing security solutions.

## How SHIGAI Works

1. **Behavioral Anomaly Detection**: SHIGAI compares the observed file behavior against learned patterns and identifies any anomalies or suspicious activities.
2. **Real-Time Alerting**: When potential ransomware behavior is detected, SHIGAI generates alerts, providing security teams with actionable information to investigate and mitigate the threat.

## Contribution Guidelines

Contributions to SHIGAI are welcome! If you would like to contribute to the project, please follow the guidelines outlined in the project's repository. Contributions can include bug fixes, feature enhancements, or the addition of new detection rules. Together, we can build a more robust and effective defense against ransomware attacks.

## Disclaimer

SHIGAI is an open-source project and should be used with caution. While it aims to enhance the security posture against ransomware, it is not a foolproof solution and should be used in conjunction with other security measures. The project's developers and maintainers take no responsibility for any damages or loss caused by the use of this software.

Join the SHIGAI community today and contribute to the fight against ransomware!
