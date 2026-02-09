package com.arvenora.lancemate.viewmodel

import androidx.compose.runtime.LaunchedEffect
import androidx.compose.runtime.mutableStateListOf
import androidx.compose.runtime.mutableStateOf
import androidx.compose.runtime.setValue
import androidx.lifecycle.ViewModel
import kotlinx.coroutines.delay
import kotlinx.coroutines.flow.MutableStateFlow
import kotlinx.coroutines.flow.StateFlow
import kotlinx.coroutines.flow.asStateFlow
import kotlinx.coroutines.flow.update

enum class ConnectionMethod {
    Bluetooth,
    USB
}

enum class ConnectionState {
    NotConnected,
    Connecting,
    Connected
}

class ConnectionManagerViewModel : ViewModel() {
    var connectionState = mutableStateOf(ConnectionState.NotConnected)
    var method = mutableStateOf(ConnectionMethod.Bluetooth)
        private set
    var showConnectionSheet = mutableStateOf(false)
    var bleDevices = mutableStateListOf("Device 1", "Device 2", "Device 3", "Device 4", "Device 5", "Device 6", "Device 7", "Device 8", "Device 9")
    var connectedDevice = mutableStateOf<String?>(null)

    fun setMethod(method: ConnectionMethod) {
        this.method.value = method
    }

    suspend fun connectToBleDevice(device: String) {
        connectionState.value = ConnectionState.Connecting
        delay(1500) // doing some work
        bleDevices.retainAll { it == device }
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