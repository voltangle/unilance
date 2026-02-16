package com.arvenora.lancemate.viewmodel

import androidx.compose.material3.SnackbarHostState
import androidx.compose.runtime.getValue
import androidx.compose.runtime.mutableStateListOf
import androidx.compose.runtime.mutableStateOf
import androidx.compose.runtime.setValue
import androidx.lifecycle.ViewModel
import com.arvenora.lancemate.ui.ConnectionManager
import kotlinx.coroutines.delay

enum class ConnectionMethod {
    BLE,
    TCP,
    USB
}

enum class ConnectionState {
    NotConnected,
    Connecting,
    Connected
}

class ConnectionManagerViewModel(val snackbarHostState: SnackbarHostState) : ViewModel() {
    var connectionState by mutableStateOf(ConnectionState.NotConnected)
    var method by mutableStateOf(ConnectionMethod.BLE)
    var showConnectionSheet by mutableStateOf(false)
    var bleDevices = mutableStateListOf("Device 1", "Device 2", "Device 3", "Device 4", "Device 5", "Device 6", "Device 7", "Device 8", "Device 9")
    var connectedDevice by mutableStateOf<String?>(null)
    var isDeviceListExpanded by mutableStateOf(false)

    suspend fun connectToBleDevice(device: String) {
        connectionState = ConnectionState.Connecting
        delay(1500) // doing some work
        isDeviceListExpanded = false
        connectedDevice = device
        connectionState = ConnectionState.Connected
        bleDevices.retainAll { it != device }
        snackbarHostState.showSnackbar("Connected to $device")
    }

    suspend fun disconnectFromBleDevice() {
        connectionState = ConnectionState.NotConnected
        snackbarHostState.showSnackbar("Disconnected from $connectedDevice")
        connectedDevice = null
    }

    fun hideSheet() {
        showConnectionSheet = false
    }

    fun showSheet() {
        showConnectionSheet = true
    }
}