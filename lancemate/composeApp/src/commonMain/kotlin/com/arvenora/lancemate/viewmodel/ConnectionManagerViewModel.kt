package com.arvenora.lancemate.viewmodel

import androidx.compose.runtime.mutableStateListOf
import androidx.compose.runtime.mutableStateOf
import androidx.lifecycle.ViewModel
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

class ConnectionManagerViewModel : ViewModel() {
    var connectionState = mutableStateOf(ConnectionState.NotConnected)
    var method = mutableStateOf(ConnectionMethod.BLE)
        private set
    var showConnectionSheet = mutableStateOf(false)
    var bleDevices = mutableStateListOf("Device 1", "Device 2", "Device 3", "Device 4", "Device 5", "Device 6", "Device 7", "Device 8", "Device 9")
    var connectedDevice = mutableStateOf<String?>(null)
    var isDeviceListExpanded = mutableStateOf(false)

    fun setMethod(method: ConnectionMethod) {
        this.method.value = method
    }

    suspend fun connectToBleDevice(device: String) {
        connectionState.value = ConnectionState.Connecting
        delay(1500) // doing some work
        isDeviceListExpanded.value = false
        connectedDevice.value = device
        connectionState.value = ConnectionState.Connected
    }

    fun hideSheet() {
        showConnectionSheet.value = false
    }

    fun showSheet() {
        showConnectionSheet.value = true
    }
}