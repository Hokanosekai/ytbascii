# YTBASCII

A Youtube client in your terminal.

## Description

YTBASCII is a Youtube client that allow you to watch video in ASCII art directly in your terminal. This service use Invidious as provider for all videos stream.

## Developpment

This project is decomposed in 3 modules, each as his own utility.

### [[Fetcher]]

The fetcher module is used to retrieve data from Invidious API, such as, videos, channels, playlists, etc, ....

### [[Core]]

The Core module is the main part of the encoding/converting of the video stream in ASCII art.

### [[UI]]

As his name this module is the UI of the client. It use [tui]() rust package.