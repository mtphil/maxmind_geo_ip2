#!/usr/bin/env groovy

pipeline {
  agent any
    stages {
        stage('Build') {
            steps {
                echo 'Building...'
                sh 'echo install'
            }
        }
        stage('Test') {
            steps {
                echo 'Testing...'
                sh 'echo testing'
            }
        }
    }
}
